use leptos::ev::Event;
use leptos::web_sys::{HtmlElement, HtmlImageElement, HtmlTextAreaElement, HtmlButtonElement};
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;
use std::cmp;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct HighlightArgs<'a> {
    code: &'a str,
    ss : SyntaxSet,
    theme : Theme,
}

#[derive(Serialize, Deserialize)]
struct LoadArgs<'a> {
    code: &'a str,
}

#[derive(Serialize, Deserialize)]
struct CustomDirEntry<> {
    path: String,
    type_of: String,
    subpaths : Vec<CustomDirEntry<>>,
}

#[component]
pub fn App() -> impl IntoView {
    // let (test, set_test) = signal(String::new());
    // let (test2, set_test2) = signal(String::new());
    let (highlighted_msg, set_highlighted_msg) = signal(String::new());

    let (syntax_set, set_syntax_set) = signal(SyntaxSet::load_defaults_nonewlines());
    let (theme, set_theme) = signal(Theme::default());

    let (sl, set_sl) = signal(0i32);
    let (st, set_st) = signal(0i32);

    let (lines_html, set_lines_html) = signal("<button>1</button>".to_string());
    let (selected_activity_icon, set_selected_activity_icon) = signal("#file-explorer-button");

    // Helper function to get width of current line of text in editor
    // Used for setting scroll_left
    fn get_width(code : &str) -> i32{
        let mut index : usize = code.len();
        for char in code.chars().rev(){
            // console_log(&char.to_string());
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
        let _ = document.query_selector("#highlighting").unwrap().unwrap().append_child(&test);

        let width = test.scroll_width();
        test.remove();
        return width;
    }


    // Helper function to get number of lines currently typed in editor
    fn get_lines(code: &str) -> String{
        let lines : Vec<&str>  = code.split("\n").collect();
        let num_lines = lines.len();
        let mut lines_html = "".to_string();
        for i in 1..num_lines+1 {
            lines_html = format!("{}{}{}{}", lines_html,"<button>",i.to_string(),"</button>");
        }
        return lines_html;
    }

    // Generate file explorer html
    fn generate_file_explorer_html(dir_entry: Vec<CustomDirEntry>) {
        for entry in dir_entry{
            let entry_type = entry.type_of;
            if entry_type == "Directory" {
                let subpaths : Vec<CustomDirEntry> = entry.subpaths;
                generate_file_explorer_html(subpaths);
            } else {
                //Do something
            }
        }
    }


    //Used for clicking line number and highlighting appropriate line of  text
    let line_number_button_closure = Closure::wrap(Box::new(move |ev: Event| {
        let button_num = ev.target().unwrap().dyn_into::<HtmlButtonElement>().unwrap().text_content().unwrap().parse::<usize>().unwrap();
        let index = button_num - 1;

        let document = leptos::prelude::document();
        let result_element = document.query_selector("#editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
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


        //console_log(lines[index]);
    }) as Box<dyn FnMut(_)>);


    // Load Syntax and Color Scheme from file
    Effect::new(move |_| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&LoadArgs { code : "null"}).unwrap();
            let return_tuple: (SyntaxSet, Theme) = serde_wasm_bindgen::from_value(invoke("load", args).await).unwrap();
            set_syntax_set.set(return_tuple.0);
            set_theme.set(return_tuple.1);
        });
    });


    view! {
        <div class="main">
            <div id="sidebar-icons">
                <div id ="temp-buffer"></div>
                <button id="file-explorer-button" class="selected"
                on:click=move |_|{
                    let currently_selected = selected_activity_icon.get();
                    let this_name = "#file-explorer-button";
                    if currently_selected == this_name {
                        let document = leptos::prelude::document();

                        let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let style = details.style();

                        if style.get_property_value("flex").unwrap() == "0 0 0px"{
                            let _ = style.set_property("flex", "0 0 200px");
                        } else if style.get_property_value("flex").unwrap() == "0 0 200px"{
                            let _ = style.set_property("flex", "0 0 0px");
                        }
                    } else {
                        let document = leptos::prelude::document();

                        let this = document.query_selector(this_name).unwrap().unwrap();
                        let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                        set_selected_activity_icon.set(this_name);

                        this.set_class_name("selected");
                        currently_selected_element.set_class_name("");
                    }
                }>
                    <img src="public/files.svg"/>
                </button>
                <button id ="account-button"
                on:click=move |_|{
                    let currently_selected = selected_activity_icon.get();
                    let this_name = "#account-button";
                    if currently_selected == this_name {
                        let document = leptos::prelude::document();

                        let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let style = details.style();
                        
                        if style.get_property_value("flex").unwrap() == "0 0 0px"{
                            let _ = style.set_property("flex", "0 0 200px");
                        } else if style.get_property_value("flex").unwrap() == "0 0 200px"{
                            let _ = style.set_property("flex", "0 0 0px");
                        }
                    } else {
                        let document = leptos::prelude::document();

                        let this = document.query_selector(this_name).unwrap().unwrap();
                        let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                        set_selected_activity_icon.set(this_name);

                        this.set_class_name("selected");
                        currently_selected_element.set_class_name("");
                    }
                }>
                    <img src="public/account.svg"/>
                </button>
                <button id ="records-button"
                on:click=move |_|{
                    let currently_selected = selected_activity_icon.get();
                    let this_name = "#records-button";
                    if currently_selected == this_name {
                        let document = leptos::prelude::document();

                        let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let style = details.style();
                        
                        if style.get_property_value("flex").unwrap() == "0 0 0px"{
                            let _ = style.set_property("flex", "0 0 200px");
                        } else if style.get_property_value("flex").unwrap() == "0 0 200px"{
                            let _ = style.set_property("flex", "0 0 0px");
                        }
                    } else {
                        let document = leptos::prelude::document();

                        let this = document.query_selector(this_name).unwrap().unwrap();
                        let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                        set_selected_activity_icon.set(this_name);

                        this.set_class_name("selected");
                        currently_selected_element.set_class_name("");
                    }
                }>
                    <img src="public/checklist.svg"/>
                </button>
                <button id ="rest-api-button"
                on:click=move |_|{
                    let currently_selected = selected_activity_icon.get();
                    let this_name = "#rest-api-button";
                    if currently_selected == this_name {
                        let document = leptos::prelude::document();

                        let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let style = details.style();
                        
                        if style.get_property_value("flex").unwrap() == "0 0 0px"{
                            let _ = style.set_property("flex", "0 0 200px");
                        } else if style.get_property_value("flex").unwrap() == "0 0 200px"{
                            let _ = style.set_property("flex", "0 0 0px");
                        }                    } else {
                        let document = leptos::prelude::document();

                        let this = document.query_selector(this_name).unwrap().unwrap();
                        let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                        set_selected_activity_icon.set(this_name);

                        this.set_class_name("selected");
                        currently_selected_element.set_class_name("");
                    }
                }>
                    <img src="public/debug-disconnect.svg"/>
                </button>
                <button id ="execute-button"
                on:click=move |_|{
                    let currently_selected = selected_activity_icon.get();
                    let this_name = "#execute-button";
                    if currently_selected == this_name {
                        let document = leptos::prelude::document();

                        let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let style = details.style();
                        
                        if style.get_property_value("flex").unwrap() == "0 0 0px"{
                            let _ = style.set_property("flex", "0 0 200px");
                        } else if style.get_property_value("flex").unwrap() == "0 0 200px"{
                            let _ = style.set_property("flex", "0 0 0px");
                        }                    } else {
                        let document = leptos::prelude::document();

                        let this = document.query_selector(this_name).unwrap().unwrap();
                        let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                        set_selected_activity_icon.set(this_name);

                        this.set_class_name("selected");
                        currently_selected_element.set_class_name("");
                    }
                }>
                    <img src="public/play-circle.svg"/>
                </button>
                <button id ="deploy-button"
                on:click=move |_|{
                    let currently_selected = selected_activity_icon.get();
                    let this_name = "#deploy-button";
                    if currently_selected == this_name {
                        let document = leptos::prelude::document();

                        let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let style = details.style();
                        
                        if style.get_property_value("flex").unwrap() == "0 0 0px"{
                            let _ = style.set_property("flex", "0 0 200px");
                        } else if style.get_property_value("flex").unwrap() == "0 0 200px"{
                            let _ = style.set_property("flex", "0 0 0px");
                        }                    
                    } else {
                        let document = leptos::prelude::document();

                        let this = document.query_selector(this_name).unwrap().unwrap();
                        let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                        set_selected_activity_icon.set(this_name);

                        this.set_class_name("selected");
                        currently_selected_element.set_class_name("");
                    }
                }>
                    <img src="public/cloud-upload.svg"/>
                </button>
                <div id ="empty-space"></div>
                <button id ="settings-button">
                    <img src="public/gear.svg"/>
                </button>

            </div>
            <div id="sidebar-details" style="flex: 0 0 200px;">
                <button id="temp-button">File Explorer</button>
                <div id="details">
                    <button id="open-folder"
                    on:click:target=move|_| {
                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&LoadArgs { code : "null"}).unwrap();

                            let return_val = invoke("open_explorer", args).await.as_string().unwrap();
                            if return_val != ""{
                                //console_log(&return_val);
                                let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
                                generate_file_explorer_html(deserialized_return_val);
                            }
                        });
                    }
                    >
                        Open Folder
                    </button>
                </div>
                <div id="temp-fs">
                    <div class="temp-dir">
                        <div class="fs-title" on:click:target=move |_| {
                            let document = leptos::prelude::document();
    
                            let test_img = document.query_selector("#test_img").unwrap().unwrap().dyn_into::<HtmlImageElement>().unwrap();
                            let children = document.query_selector(".dir-children").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            if test_img.class_name() == "inactive"{
                                test_img.set_src("public/chevron-down.svg");
                                test_img.set_class_name("active");
                                children.style().set_property("display", "flex");
                            } else {
                                test_img.set_src("public/chevron-right.svg");
                                test_img.set_class_name("inactive");
                                children.style().set_property("display", "none");
                            }
                        }>
                            <img id="test_img" class="inactive" src="public/chevron-right.svg"/>
                            <div>this_is_a_dir</div>
                        </div>
                        <div class="dir-children">
                            <div class="file">
                                <div class="fs-title">
                                    <img id="test_img" src="public/file.svg"/>
                                    <div>this_is_a_file</div>
                                </div>
                            </div>

                            <div class="temp-dir">
                                <div class="fs-title" on:click:target=move |_| {
                                    let document = leptos::prelude::document();
            
                                    let test_img = document.query_selector("#test_img2").unwrap().unwrap().dyn_into::<HtmlImageElement>().unwrap();
                                    let children = document.query_selector("#childtest").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();

                                    if test_img.class_name() == "inactive"{
                                        test_img.set_src("public/chevron-down.svg");
                                        test_img.set_class_name("active");
                                        children.style().set_property("display", "flex");
                                    } else {
                                        test_img.set_src("public/chevron-right.svg");
                                        test_img.set_class_name("inactive");
                                        children.style().set_property("display", "none");
                                    }
                                }>
                                    <img id="test_img2" class="inactive" src="public/chevron-right.svg"/>
                                    <div>this_is_a_dir</div>
                                </div>
                                <div id="childtest" class="dir-children">
                                    <div class="file">
                                        <div class="fs-title">
                                            <img src="public/file.svg"/>
                                            <div>this_is_a_file</div>
                                        </div>
                                    </div>
        
        
        
        
        
                                    
                                </div>
                            </div>




                        </div>
                    </div>



                // <div id="line-numbers" inner_html={ move || lines_html.get() }></div>
                // {Effect::new(move |_| {
                //     let _signal = lines_html.get();
                //     let document = leptos::prelude::document();
                //     let result_element = document.query_selector("#line-numbers").unwrap().unwrap();
                //     let children = result_element.children();
                //     for i in 0..children.length(){
                //         let child = children.get_with_index(i).unwrap();
                //         let _ = child.add_event_listener_with_callback("click", line_number_button_closure.as_ref().unchecked_ref());
                //     }
                // });}
                </div>

            </div>
            <div class= "code-terminal-area">
                <div class= "outer-code-area">
                    <div class= "tabs"></div>
                    <div class= "ide">
                        <div id="line-numbers" inner_html={ move || lines_html.get() }></div>
                        {Effect::new(move |_| {
                            let _signal = lines_html.get();
                            let document = leptos::prelude::document();
                            let result_element = document.query_selector("#line-numbers").unwrap().unwrap();
                            let children = result_element.children();
                            for i in 0..children.length(){
                                let child = children.get_with_index(i).unwrap();
                                let _ = child.add_event_listener_with_callback("click", line_number_button_closure.as_ref().unchecked_ref());
                            }
                        });}
                        <div class="editor">
                            <textarea id="editing" 
                            spellcheck="false"
                            autocomplete="off"
                            on:scroll:target=move |ev| {
                                set_sl.set(ev.target().scroll_left());
                                set_st.set(ev.target().scroll_top());

                                let document = leptos::prelude::document();
                                let result_element = document.query_selector("#line-numbers").unwrap().unwrap();
                                result_element.set_scroll_top(st.get());
                                result_element.set_scroll_left(sl.get());
                            }
                            on:input:target=move |ev| {
                                let mut code = ev.target().value();
                                let previous_index = cmp::max(0, (code.len() as isize)-1) as usize;
                                let last_char = &code[previous_index..code.len()];
                                if last_char == "\n" {
                                    code = format!("{}{}", code, "\u{00A0}");   
                                }
                                let lines_html = get_lines(&code);
                                set_lines_html.set(lines_html);


                                spawn_local(
                                    async move {
                                        let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &code, ss : syntax_set.get_untracked(), theme : theme.get_untracked()}).unwrap();
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
                                if key == "Tab" {
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
                            ></textarea>
                            <pre id="highlighting" aria-hidden="true">
                                {Effect::new(move |_| {
                                    let document = leptos::prelude::document();
                                    let result_element = document.query_selector("#highlighting").unwrap().unwrap();
                                    result_element.set_scroll_top(st.get());
                                    result_element.set_scroll_left(sl.get());
                                });}
            
                                <div id="highlighting-content" inner_html={ move || highlighted_msg.get() }></div>
                            </pre>
                        </div>
                    </div>
                </div>
                <div class = "terminal"></div>
            </div>

        </div>
    
    }
}
