use leptos::ev::Event;
use leptos::web_sys::{HtmlElement, HtmlTextAreaElement, HtmlButtonElement};
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;
use std::cmp;
use std::collections::HashMap;




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
    syntax_set: ReadSignal<SyntaxSet>,
    theme: ReadSignal<Theme>,
    selected_file : ReadSignal<String>,
    saved_file_contents: ReadSignal<HashMap<String,String>>, 
    set_saved_file_contents: WriteSignal<HashMap<String,String>>,
    cached_file_contents: ReadSignal<HashMap<String,String>>, 
    set_cached_file_contents: WriteSignal<HashMap<String,String>>
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
                            let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &ev.target().value(), ss : syntax_set.get_untracked(), theme : theme.get_untracked()}).unwrap();
                            let highlighted = invoke("highlight", args).await.as_string().unwrap();
                            

                            set_highlighted_msg.set(highlighted);
                            set_sl.set(ev.target().scroll_left());
                            set_st.set(ev.target().scroll_top());
                            
                        }
                    );
                    
                }
                on:keydown:target= move |ev| {
                    // This function is used for the special case of Tabs, as HtmlTextArea does not natively support tab.
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
                    }
                    set_key_pressed.set(key_pressed_map);

                    if &key == "Tab" {
                        /* Tab key pressed */
                        ev.prevent_default(); // stop normal

                        let selection_start = ev.target().selection_start().unwrap().unwrap() as usize;
                        let selection_end = ev.target().selection_end().unwrap().unwrap() as usize;
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
                                let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &new_code, ss : syntax_set.get_untracked(), theme : theme.get_untracked()}).unwrap();
                                let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                set_highlighted_msg.set(highlighted);

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
                on:keyup:target= move |ev| {
                    let key = ev.key();
                    let mut key_pressed_map = key_pressed.get();
                    key_pressed_map.remove(&key);
                    set_key_pressed.set(key_pressed_map);

                }
                ></textarea>


                {Effect::new(move |_| {
                    let selected = selected_file.get();
                    if selected != String::new(){
                        let mut saved_contents = saved_file_contents.get_untracked();
                        let mut cached_contents = cached_file_contents.get_untracked();
                        if cached_contents.contains_key(&selected){
                            spawn_local(
                                async move {
                                    let contents = cached_contents.get(&selected).unwrap();
                                    let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : syntax_set.get_untracked(), theme : theme.get_untracked()}).unwrap();
                                    let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                    set_highlighted_msg.set(highlighted);


                                    let document = leptos::prelude::document();
                                    let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                    result_element.set_value(&contents);

                                    let lines_html = get_lines(contents.to_string());
                                    set_lines_html.set(lines_html);

                                    let result_element = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = result_element.style().remove_property("display");

                                }
                            );
                        } else {
                            spawn_local(
                                async move {
                                    let args = serde_wasm_bindgen::to_value(&ReadFileArgs{filepath: selected.clone()}).unwrap();
                                    match invoke("read_file", args).await.as_string(){
                                        Some(contents) => {
                                            let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : syntax_set.get_untracked(), theme : theme.get_untracked()}).unwrap();
                                            let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                            set_highlighted_msg.set(highlighted);

                                            saved_contents.insert(selected.clone(), contents.clone());
                                            cached_contents.insert(selected, contents.clone());
                                            set_saved_file_contents.set(saved_contents);
                                            set_cached_file_contents.set(cached_contents);

                                            let document = leptos::prelude::document();
                                            let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                            result_element.set_value(&contents);

                                            let lines_html = get_lines(contents);
                                            set_lines_html.set(lines_html);

                                            let result_element = document.query_selector(".ide").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                            let _ = result_element.style().remove_property("display");
                                        },
                                        None => {
                                            console_log("Error: File does not exist");
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