use leptos::ev::Event;
use leptos::web_sys::{Element, HtmlElement, HtmlTextAreaElement, HtmlButtonElement};
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;
use std::cmp;
use std::collections::HashMap;
use std::path::Path;
use js_sys::Array;
use regex::Regex;
use web_sys::css::escape;

use crate::app::{generate_file_explorer_html, CustomDirEntry};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}



/*
==============================================================================
STRUCTS
==============================================================================
*/
#[derive(Serialize, Deserialize)]
struct HighlightArgs<'a> {
    code: &'a str,
    ss : SyntaxSet,
    theme : Theme,
    filetype : String,
}


#[derive(Serialize, Deserialize)]
struct ReadFileArgs<> {
    filepath: String
}

#[derive(Serialize, Deserialize)]
struct WriteFileArgs<> {
    filepath : String,
    contents : String
}

#[derive(Serialize, Deserialize)]
pub struct Command<> {
    pub command : Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct GetDirArgs<> {
    pub directory : String
}

/*
==============================================================================
HELPER FUNCTIONS
==============================================================================
*/

// Helper function to get width of current line of text in editor
// Used for setting scroll_left
fn get_width(code : &str) -> i32{
    let mut index : usize = code.len();
    for char in code.chars().rev(){
        if char == '\n' {
            break;
        } else {
            index -= 1
        }
    }
    let before_tab_no_newline = &code[index..code.len()];
    let new_code = format!("{}{}", before_tab_no_newline, "<\u{0009}>");

    let document = leptos::prelude::document();
    let test = document.create_element("span").unwrap();
    let _ = test.set_attribute("id", "length-test");
    test.set_text_content(Some(&new_code));
    let _ = document.query_selector(".highlighting").unwrap().unwrap().append_child(&test);

    let width = test.scroll_width();
    test.remove();
    return width;
}


// Helper function to get number of lines currently typed in editor
pub fn get_lines(code2: String) -> String{
    let mut code = code2;
    let last_index = cmp::max(0, (code.len() as isize)-1) as usize;
    let last_char = &code[last_index..code.len()];
    if last_char == "\n" {
        code = format!("{}{}", code, "\u{00A0}");   
    }
    let lines : Vec<&str>  = code.split("\n").collect();
    let num_lines = lines.len();
    let mut lines_html = "".to_string();
    for i in 1..num_lines+1 {
        lines_html = format!("{}{}{}{}", lines_html,"<button>",i.to_string(),"</button>");
    }
    return lines_html;
}

/*
==============================================================================
COMPONENTS
==============================================================================
*/

#[component]
pub fn IDE(
    lines_html: ReadSignal<String>,
    set_lines_html : WriteSignal<String>,
    sl : ReadSignal<i32>,
    set_sl : WriteSignal<i32>,
    st : ReadSignal<i32>,
    set_st : WriteSignal<i32>,
    highlighted_msg : ReadSignal<String>,
    set_highlighted_msg : WriteSignal<String>,
    leo_syntax_set: ReadSignal<SyntaxSet>,
    leo_theme: ReadSignal<Theme>,
    aleo_syntax_set: ReadSignal<SyntaxSet>,
    aleo_theme: ReadSignal<Theme>,     
    selected_file : ReadSignal<String>,
    saved_file_contents: ReadSignal<HashMap<String,String>>, 
    set_saved_file_contents: WriteSignal<HashMap<String,String>>,
    cached_file_contents: ReadSignal<HashMap<String,String>>, 
    set_cached_file_contents: WriteSignal<HashMap<String,String>>,
    set_compiled_project : WriteSignal<(String,String)>,
    current_environment_dropdown_item : ReadSignal<String>,
    set_fs_html : WriteSignal<String>,
    root : ReadSignal<String>,
) -> impl IntoView {


    //Used for clicking line number and highlighting appropriate line of  text
    let line_number_button_closure = Closure::wrap(Box::new(move |ev: Event| {
        let button_num = ev.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap().text_content().unwrap().parse::<usize>().unwrap();
        let index = button_num - 1;

        let document = leptos::prelude::document();
        let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
        let code = result_element.value();
        let lines : Vec<&str> = code.split("\n").collect();

        let mut line_start_index = 0;
        for i in 0..index{
            line_start_index += lines[i].len() + 1;
        }
        let line_end_index = line_start_index + lines[index].len();
        let _ = result_element.set_selection_start(Some(line_start_index as u32));
        let _ = result_element.set_selection_end(Some(line_end_index as u32));
        let _ = result_element.focus();

    }) as Box<dyn FnMut(_)>);

    let (key_pressed, set_key_pressed) : (ReadSignal<HashMap<String,bool>>,WriteSignal<HashMap<String,bool>>) = signal(HashMap::new());

    view! {
        <div class= "ide">
            <div class="ide-error" style="display:none; width:100%; height:100%">
                <img src="public/error.svg"/> 
                <div class="ide-error-text">Error: File not found</div>
            </div>
            <div class="line-numbers" inner_html={ move || lines_html.get() }></div>
            {Effect::new(move |_| {
                let _signal = lines_html.get();
                let document = leptos::prelude::document();
                let result_element = document.query_selector(".line-numbers").unwrap().unwrap();
                let children = result_element.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    let _ = child.add_event_listener_with_callback("click", line_number_button_closure.as_ref().unchecked_ref());
                }
            });}
            <div class="editor">
                <textarea class="editing" 
                spellcheck="false"
                autocomplete="off"
                on:scroll:target=move |ev| {
                    set_sl.set(ev.target().scroll_left());
                    set_st.set(ev.target().scroll_top());

                    let document = leptos::prelude::document();
                    let result_element = document.query_selector(".line-numbers").unwrap().unwrap();
                    result_element.set_scroll_top(st.get());
                    result_element.set_scroll_left(sl.get());
                }
                on:input:target=move |ev| {
                    let lines_html = get_lines(ev.target().value());
                    set_lines_html.set(lines_html);

                    let mut cached_content = cached_file_contents.get_untracked();
                    let current_filepath = selected_file.get_untracked();
                    cached_content.remove(&current_filepath);
                    cached_content.insert(current_filepath,  ev.target().value());
                    set_cached_file_contents.set(cached_content);   

                    spawn_local(
                        async move {
                            let selected_filepath = selected_file.get_untracked().replace("\\", "/");
                            let path = Path::new(&selected_filepath);
                            let extension = path.extension();
    
                            match extension {
                                Some(ext) => {
                                    if ext.to_str().expect("Error parsing extension to string") == "leo" {
                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                        
            
                                        set_highlighted_msg.set(highlighted);
                                        set_sl.set(ev.target().scroll_left());
                                        set_st.set(ev.target().scroll_top());
                                    } else if ext.to_str().expect("Error parsing extension to string") == "aleo" {
                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : aleo_syntax_set.get_untracked(), theme : aleo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                        
            
                                        set_highlighted_msg.set(highlighted);
                                        set_sl.set(ev.target().scroll_left());
                                        set_st.set(ev.target().scroll_top());
          
                                    } else {
                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                        
            
                                        set_highlighted_msg.set(highlighted);
                                        set_sl.set(ev.target().scroll_left());
                                        set_st.set(ev.target().scroll_top());
                                    }
                                }
                                None => {
                                    let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                    let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                    
        
                                    set_highlighted_msg.set(highlighted);
                                    set_sl.set(ev.target().scroll_left());
                                    set_st.set(ev.target().scroll_top());
                                }
                            }
                        }
                    );
                    
                }
                on:keydown:target= move |ev| {
                    let key = ev.key();
                    let code = ev.target().value();

                    let mut key_pressed_map = key_pressed.get();
                    key_pressed_map.insert(key.clone(),true);

                    if (&key_pressed_map).contains_key("Control") && &key == "s" {
                        let current_filepath = selected_file.get_untracked();
                        let mut saved_content = saved_file_contents.get_untracked();
                        let mut cached_content = cached_file_contents.get_untracked();
                        let current_saved_file_content = saved_content.get(&current_filepath).unwrap();

                        let document = leptos::prelude::document();
                        let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

                        
                        if &result_element.value() != current_saved_file_content{
                            spawn_local(
                                async move {
                                    let document = leptos::prelude::document();
                                    let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

                                    let args = serde_wasm_bindgen::to_value(&WriteFileArgs { filepath: current_filepath.clone(), contents: result_element.value()}).unwrap();
                                    let (error, message) : (bool, String) = serde_wasm_bindgen::from_value(invoke("write_file", args).await).unwrap();
                                    if !error {
                                        saved_content.remove(&current_filepath);
                                        saved_content.insert(current_filepath.clone(), result_element.value());
                                        set_saved_file_contents.set(saved_content);

                                        cached_content.remove(&current_filepath);
                                        cached_content.insert(current_filepath, result_element.value());
                                        set_cached_file_contents.set(cached_content);                     
                                    } else {
                                        console_log(&message);
                                    }
                                }
                            );
                        }

                        //Compile file if main.leo

                        let selected_filepath = selected_file.get().replace("\\", "/");
                        let path = Path::new(&selected_filepath);
                        let filename = path.file_name();

                        let document = leptos::prelude::document();
                        match filename {
                            Some(name) => {
                                if name.to_str().unwrap() == "main.leo" {
                                    let src = path.parent().unwrap();
                                    let project_root = src.parent().unwrap();
                                    let compile_project_root = (project_root.to_str().unwrap().to_string(),project_root.file_name().unwrap().to_str().unwrap().to_string());
                                    
                                    let target = document.query_selector("#compiler-output").unwrap().unwrap();
                                    let _ = target.set_class_name("compile-output-message");
                                    target.set_inner_html("");
            
                                    let this = target.dyn_into::<Element>().unwrap();
                                    let new_val = Array::new();
                                    new_val.push(&serde_wasm_bindgen::to_value("disabled").unwrap());
                                    let _ = this.class_list().add(&new_val);
            
                                    spawn_local(async move{
                                        let target = document.query_selector("#compiler-output").unwrap().unwrap();

                                        let network : String = if current_environment_dropdown_item.get_untracked() == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};
                                        let args = serde_wasm_bindgen::to_value(&Command { command : vec!["build".to_string(), "--path".to_string(), compile_project_root.0.clone(), "--network".to_string(), network ,"--endpoint".to_string(),"https://api.explorer.provable.com/v1".to_string()]}).unwrap();
                
                                        let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
            
                                        if !error {
                                            let args = serde_wasm_bindgen::to_value(&GetDirArgs { directory : root.get_untracked()}).unwrap();
                                            let return_val = invoke("get_directory", args).await.as_string().unwrap();
                                            let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
                                            let html_fs = generate_file_explorer_html(deserialized_return_val);
                                            set_fs_html.set(html_fs);
            
                                            let _ = target.set_class_name("compile-output-message success");
                                            let output = format!("{}{}{}", "\u{2713} Compiled '", compile_project_root.1.clone(), "' into Aleo instructions!");
                                            target.set_inner_html(&output);
            
                                            set_compiled_project.set(compile_project_root);
            
                                        } else {
                                            let _ = target.set_class_name("compile-output-message failure");
                                            
                                            let split = output.split("\n").collect::<Vec<&str>>();
                                            let error_msg = split[0];
            
                                            let re = Regex::new(r"Error \[(?<error_code>[a-zA-Z0-9]*)\]:(?<explanation>.*)").unwrap();
                                            match re.captures(error_msg) {
                                                Some(caps) => {
                                                    let response = format!("{}{}{}{}","Error ".to_string(), caps["error_code"].to_string(), ":",  caps["explanation"].to_string());
                                                    target.set_inner_html(&response);
                                                },
                                                None => {
                                                    target.set_inner_html("Unknown Error");
                                                }
                                            }
            
                                        }
                                        let _ = this.class_list().remove(&new_val);
                                    });
                        
                                
                                } 
                            },
                            None => {}
                        }
                    } else if &key == "Tab" {
                        // This function is used for the special case of Tabs, as HtmlTextArea does not natively support tab.
                        /* Tab key pressed */
                        ev.prevent_default(); // stop normal

                        let selection_start = ev.target().selection_start().unwrap().unwrap() as usize;
                        let selection_end = ev.target().selection_end().unwrap().unwrap() as usize;
                        if selection_start == selection_end {
                            let before_tab = &code[0..selection_start];
                            let after_tab = &code[selection_end..code.len()];
                            let width_check = get_width(before_tab);
                            let new_code = format!("{}{}{}", before_tab, "\u{0009}", after_tab);                                                
                            ev.target().set_value(&new_code);
    
                            let cursor_pos = (selection_end + 1) as u32;
                            let _ = ev.target().set_selection_start(Some(cursor_pos));
                            let _ = ev.target().set_selection_end(Some(cursor_pos));
    
    
                            spawn_local(
                                async move {
                                    let selected_filepath = selected_file.get_untracked().replace("\\", "/");
                                    let path = Path::new(&selected_filepath);
                                    let extension = path.extension();
            
                                    match extension {
                                        Some(ext) => {
                                            if ext.to_str().expect("Error parsing extension to string") == "leo" {
                                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                set_highlighted_msg.set(highlighted);
                                            } else if ext.to_str().expect("Error parsing extension to string") == "aleo" {
                                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : aleo_syntax_set.get_untracked(), theme : aleo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                set_highlighted_msg.set(highlighted);
                                            } else {
                                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                set_highlighted_msg.set(highlighted);
                                            }
                                        }
                                        None => {
                                            let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                            let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                            set_highlighted_msg.set(highlighted);
                                        }
                                    }
    
                                    let scroll_left = ev.target().scroll_left();
                                    let client_width =  ev.target().client_width();
                                    if client_width + scroll_left <  width_check {
                                        let new_scroll_distance = scroll_left + (width_check - (client_width + scroll_left));
                                        ev.target().set_scroll_left(new_scroll_distance);
                                        set_sl.set(new_scroll_distance);    
                                    }  
            
                                }
                            );
                        }
                    } else if &key == "Enter" {
                        ev.prevent_default();

                        let selection_start = ev.target().selection_start().unwrap().unwrap() as usize;
                        let selection_end = ev.target().selection_end().unwrap().unwrap() as usize;
                        if selection_start == selection_end {
                            let before_this = &code[0..selection_start];
                            let after_this = &code[selection_end..code.len()];

                            let before_this_lines = before_this.split("\n").collect::<Vec<&str>>();
                            let this_line = before_this_lines[before_this_lines.len()-1];

                            let mut num_tabs = 0;
                            for charc in this_line.chars(){
                                if charc == '\u{0009}' {
                                    num_tabs += 1;
                                }
                            }
                            let tabs = "\u{0009}".repeat(num_tabs);
                            let width_check = get_width(before_this);
                            let new_code = format!("{}{}{}{}", before_this,"\n",tabs, after_this);                                                
                            ev.target().set_value(&new_code);
    
                            let cursor_pos = (selection_end + num_tabs + 1) as u32;
                            let _ = ev.target().set_selection_start(Some(cursor_pos));
                            let _ = ev.target().set_selection_end(Some(cursor_pos));
    
                            let lines_html = get_lines(new_code.clone());
                            set_lines_html.set(lines_html);

                            spawn_local(
                                async move {
                                    let selected_filepath = selected_file.get_untracked().replace("\\", "/");
                                    let path = Path::new(&selected_filepath);
                                    let extension = path.extension();
            
                                    match extension {
                                        Some(ext) => {
                                            if ext.to_str().expect("Error parsing extension to string") == "leo" {
                                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &new_code, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                set_highlighted_msg.set(highlighted);
                                            } else if ext.to_str().expect("Error parsing extension to string") == "aleo" {
                                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &new_code, ss : aleo_syntax_set.get_untracked(), theme : aleo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                set_highlighted_msg.set(highlighted);
                                            } else {
                                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &new_code, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                set_highlighted_msg.set(highlighted);
                                            }
                                        }
                                        None => {
                                            let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &new_code, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                            let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                            set_highlighted_msg.set(highlighted);
                                        }
                                    }

    
                                    let scroll_left = ev.target().scroll_left();
                                    let client_width =  ev.target().client_width();
                                    if client_width + scroll_left <  width_check {
                                        let new_scroll_distance = scroll_left + (width_check - (client_width + scroll_left));
                                        ev.target().set_scroll_left(new_scroll_distance);
                                        set_sl.set(new_scroll_distance);    
                                    }  
            
                                }
                            );
                        }

                    }


                    set_key_pressed.set(key_pressed_map);
                }
                on:keyup:target= move |ev| {
                    let key = ev.key();
                    let mut key_pressed_map = key_pressed.get();
                    key_pressed_map.remove(&key);
                    set_key_pressed.set(key_pressed_map);

                }
                >
                </textarea>


                {Effect::new(move |_| {
                    let document = leptos::prelude::document();

                    let selected = selected_file.get();
                    if selected != String::new(){
                        let mut saved_contents = saved_file_contents.get_untracked();
                        let mut cached_contents = cached_file_contents.get_untracked();
                        if cached_contents.contains_key(&selected){
                            spawn_local(
                                async move {
                                    let args = serde_wasm_bindgen::to_value(&ReadFileArgs{filepath: selected.clone()}).unwrap();
                                    match invoke("read_file", args).await.as_string(){
                                        Some(contents) => {
                                            let cached_content = cached_contents.get(&selected).unwrap().to_string();

                                            let selected_filepath = selected_file.get_untracked().replace("\\", "/");
                                            let path = Path::new(&selected_filepath);
                                            let extension = path.extension();
                    
                                            match extension {
                                                Some(ext) => {
                                                    if ext.to_str().expect("Error parsing extension to string") == "leo" {
                                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &cached_content, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                        set_highlighted_msg.set(highlighted);
                                                    } else if ext.to_str().expect("Error parsing extension to string") == "aleo" {
                                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &cached_content, ss : aleo_syntax_set.get_untracked(), theme : aleo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                        set_highlighted_msg.set(highlighted);
                                                    } else {
                                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &cached_content, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                        set_highlighted_msg.set(highlighted);
                                                    }
                                                }
                                                None => {
                                                    let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &cached_content, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                                    let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                    set_highlighted_msg.set(highlighted);
                                                }
                                            }

                                            saved_contents.insert(selected.clone(), contents.clone());
                                            set_saved_file_contents.set(saved_contents);

                                            let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                            result_element.set_value(&cached_content);

                                            let lines_html = get_lines(cached_content);
                                            set_lines_html.set(lines_html);

                                            let result_element1 = document.query_selector(".ide-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element1.style().set_property("display","none");

                                            let result_element2 = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element2.style().remove_property("display");

                                            let result_element3 = document.query_selector(".editor").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element3.style().remove_property("display");
                                            
                                            let result_element4 = document.query_selector(".line-numbers").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element4.style().remove_property("display");
                                        },
                                        None => {
                                            let result_element1 = document.query_selector(".ide-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element1.style().remove_property("display");

                                            let tab_id = format!("{}{}", "#", escape(&selected.clone()));
                                            let tab_element = document.query_selector(&tab_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = tab_element.set_attribute("valid", "false");

                                            let result_element2 = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element2.style().remove_property("display");

                                            let result_element3 = document.query_selector(".editor").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element3.style().set_property("display", "none");
                                            
                                            let result_element4 = document.query_selector(".line-numbers").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element4.style().set_property("display", "none");

                                        }
                                    }

                                }
                            );
                        } else {
                            spawn_local(
                                async move {
                                    let args = serde_wasm_bindgen::to_value(&ReadFileArgs{filepath: selected.clone()}).unwrap();

                                    match invoke("read_file", args).await.as_string(){
                                        Some(contents) => {
                                            let selected_filepath = selected_file.get_untracked().replace("\\", "/");
                                            let path = Path::new(&selected_filepath);
                                            let extension = path.extension();
                    
                                            match extension {
                                                Some(ext) => {
                                                    if ext.to_str().expect("Error parsing extension to string") == "leo" {
                                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                        set_highlighted_msg.set(highlighted);
                                                    } else if ext.to_str().expect("Error parsing extension to string") == "aleo" {
                                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : aleo_syntax_set.get_untracked(), theme : aleo_theme.get_untracked(), filetype : ext.to_str().unwrap().to_string()}).unwrap();
                                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                        set_highlighted_msg.set(highlighted);
                                                    } else {
                                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                                        let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                        set_highlighted_msg.set(highlighted);
                                                    }
                                                }
                                                None => {
                                                    let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : leo_syntax_set.get_untracked(), theme : leo_theme.get_untracked(), filetype : "default".to_string() }).unwrap();
                                                    let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                    set_highlighted_msg.set(highlighted);
                                                }
                                            }

                                            saved_contents.insert(selected.clone(), contents.clone());
                                            cached_contents.insert(selected, contents.clone());
                                            set_saved_file_contents.set(saved_contents);
                                            set_cached_file_contents.set(cached_contents);

                                            let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                            result_element.set_value(&contents);

                                            let lines_html = get_lines(contents);
                                            set_lines_html.set(lines_html);

                                            let result_element1 = document.query_selector(".ide-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element1.style().set_property("display","none");

                                            let result_element2 = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element2.style().remove_property("display");

                                            let result_element3 = document.query_selector(".editor").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element3.style().remove_property("display");
                                            
                                            let result_element4 = document.query_selector(".line-numbers").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element4.style().remove_property("display");
                                        },
                                        None => {
                                            let result_element1 = document.query_selector(".ide-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element1.style().remove_property("display");


                                            let tab_id = format!("{}{}", "#", escape(&selected.clone()));
                                            let tab_element = document.query_selector(&tab_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = tab_element.set_attribute("valid", "false");

                                            let result_element2 = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element2.style().remove_property("display");

                                            let result_element3 = document.query_selector(".editor").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element3.style().set_property("display", "none");
                                            
                                            let result_element4 = document.query_selector(".line-numbers").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element4.style().set_property("display", "none");

                                        }
                                    }
                                }
                            );
                        }
                    } else {
                        let document = leptos::prelude::document();
                        let result_element = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let _ = result_element.style().set_property("display","none");
                    }
                });}


                <pre class="highlighting" aria-hidden="true">
                    {Effect::new(move |_| {
                        let document = leptos::prelude::document();
                        let result_element = document.query_selector(".highlighting").unwrap().unwrap();
                        result_element.set_scroll_top(st.get());
                        result_element.set_scroll_left(sl.get());
                    });}

                    <div class="highlighting-content" inner_html={ move || highlighted_msg.get() }></div>
                </pre>
            </div>
        </div>
    }
}