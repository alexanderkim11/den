use leptos::ev::Event;
use leptos::web_sys::{Element,HtmlElement, HtmlImageElement, HtmlTextAreaElement, HtmlButtonElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;
use web_sys::css::escape;
use std::cmp;
use std::fs::File;

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
struct LoadArgs<'a> {
    code: &'a str,
}

#[derive(Serialize, Deserialize)]
struct CustomDirEntry<> {
    path: String,
    name: String,
    type_of: String,
    subpaths : Vec<CustomDirEntry<>>,
}

struct RecursiveClosureWrapper<'s>  {
    placeholder: Element,
    closure : &'s dyn Fn(Element,&RecursiveClosureWrapper)
}


/*
==============================================================================
HELPER FUNCTIONS
==============================================================================
*/



// Recursively generate file explorer html
fn generate_file_explorer_html(mut dir_entry: Vec<CustomDirEntry>) -> String {
    let mut fs_html : String = "".to_string();
    dir_entry.sort_unstable_by_key(|entry| (entry.type_of.clone(), entry.name.clone()));
    for entry in dir_entry{
        let entry_type = entry.type_of;
        if entry_type == "Directory" {
            let subpaths : Vec<CustomDirEntry> = entry.subpaths;
            let fs_title = format!("{}{}{}{}{}", "<div name = \"title\" class=\"fs-title\"><img name = \"image\" id=\"", entry.path, "--img\" class=\"inactive\" src=\"public/chevron-right.svg\"/><div>", entry.name, "</div></div>");
            let dir_children = format!("{}{}{}{}{}", "<div name = \"children\" id=\"",entry.path,"--children\" class=\"dir-children\">", generate_file_explorer_html(subpaths), "</div>");
            fs_html = format!("{}{}{}{}{}", fs_html,"<div class=\"dir\">", fs_title, dir_children, "</div>");

        } else if entry_type == "File" {
            fs_html = format!("{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/file.svg\"/><div>", entry.name, "</div></div></div>");
        }
    }
    return fs_html;

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


/*
==============================================================================
AUXILLARY COMPONENTS
==============================================================================
*/


#[component]
fn SidebarIcon(
    #[prop(optional)]
    selected: bool,
    id : String,
    img_src : String,
    selected_activity_icon: ReadSignal<String>,
    set_selected_activity_icon : WriteSignal<String>
) -> impl IntoView {

    let class : &str;
    if selected{
        class = "selected";
    } else {
        class = "";
    }
    view!{
        <button id = id.clone() class=class
        on:click=move |_|{
            let currently_selected = selected_activity_icon.get();
            let this_name = format!("{}{}","#", id);
            if currently_selected == this_name {
                let document = leptos::prelude::document();

                let details = document.query_selector(".sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                let style = details.style();

                if style.get_property_value("display").unwrap() == "flex"{
                    let _ = style.set_property("display", "none");
                } else if style.get_property_value("display").unwrap() == "none"{
                    let _ = style.set_property("display", "flex");
                }
            } else {
                let document = leptos::prelude::document();

                let this = document.query_selector(&this_name).unwrap().unwrap();
                let currently_selected_element = document.query_selector(&currently_selected).unwrap().unwrap();
                set_selected_activity_icon.set(this_name);

                this.set_class_name("selected");
                currently_selected_element.set_class_name("");
            }
        }>
            <img src=img_src/>
        </button>
    }
}


#[component]
fn FileTab(
    filepath: String,
    filename: String,
    selected_file: ReadSignal<String>,
    set_selected_file : WriteSignal<String>
) -> impl IntoView {
    
    view! {
        <button id=filepath.clone() class="tab"
        on:click:target = move|ev|{
            let target = ev.target().dyn_into::<Element>().unwrap();
            let new_val = Array::new();
            new_val.push(&serde_wasm_bindgen::to_value("selected").unwrap());
            let _ = target.class_list().add(&new_val);
            set_selected_file.set(target.id());
        }>
            {filename}
        </button>
        {Effect::new(move |_| {
            let document = leptos::prelude::document();
            let target_string = format!("{}{}", "#", escape(&filepath));
            let target2 = document.query_selector(&target_string).unwrap();
            match target2 {
                Some(e) => {
                    let target = e;
                    if selected_file.get() != target.id(){
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("selected").unwrap());
                        let _ = target.class_list().remove(&new_val);
                    }
                }
                None => {}

            }
        });
        }

    }
}


/*
==============================================================================
MAIN APP COMPONENT
==============================================================================
*/


#[component]
pub fn App() -> impl IntoView {
    //let (test, set_test) = signal(String::new());
    let (highlighted_msg, set_highlighted_msg) = signal(String::new());

    let (syntax_set, set_syntax_set) = signal(SyntaxSet::load_defaults_nonewlines());
    let (theme, set_theme) = signal(Theme::default());

    let (sl, set_sl) = signal(0i32);
    let (st, set_st) = signal(0i32);

    let (sidebar_offset_x, set_sidebar_offset_x) = signal(0i32);
    let (sidebar_dragging, set_sidebar_dragging) = signal(false);

    let (lines_html, set_lines_html) = signal("<button>1</button>".to_string());
    let (fs_html, set_fs_html) = signal(String::new());

    let (selected_activity_icon, set_selected_activity_icon) = signal("#file-explorer-button".to_string());
    let (selected_file, set_selected_file) = signal(String::new());
    let new_vec : Vec<(String,String)> = Vec::new();
    // let new_vec : Vec<(String,String)> = vec![("test".to_string(), "test2".to_string())];
    let (open_files, set_open_files) = signal(new_vec.clone());

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


    //Used for switching the chevron icon next to directories in the file system tab
    let switch_chevron_closure = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let children = this.children();
        let dir_element = this.parent_element().unwrap();


        let img = children.named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let dir_children = dir_element.children().named_item("children").unwrap().dyn_into::<HtmlElement>().unwrap();
        if img.class_name() == "inactive"{
            img.set_src("public/chevron-down.svg");
            img.set_class_name("active");
            let _ = dir_children.style().set_property("display", "flex");
        } else {
            img.set_src("public/chevron-right.svg");
            img.set_class_name("inactive");   
            let _ = dir_children.style().set_property("display", "none");
        }
    }) as Box<dyn FnMut(_)>);

    


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

    //Used for opening files
    let open_file_closure = Closure::wrap(Box::new(move |ev: Event| {
        let title_element = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        let dataset = title_element.dataset();
        let filepath = dataset.get("filepath").unwrap();
        let temp = filepath.clone();
        let collection = temp.split("\\").collect::<Vec<&str>>();
        let filename = collection[collection.len()-1].to_string();

        set_selected_file.set(filepath.clone());
        set_open_files.update(|vec| vec.push((filepath,filename)));
        /*
        TODO:
            - Only update files list if file is not already open
            - Add exit button to close file tab
            - Actually read file content and put into editor
            - Reset FS default directory
            - Highlight current line of text with gray
         */

        
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
        <div class="main" 
        on:mousemove:target=move|ev|{
            if sidebar_dragging.get() {
                ev.prevent_default();
                let document = leptos::prelude::document();
                let x = ev.screen_x() - sidebar_offset_x.get();
                let result_element = document.query_selector(".sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                let style = result_element.style();


                let raw_val = style.get_property_value("flex-basis").unwrap();
                let val = raw_val[0..(raw_val.len()-2)].parse::<i32>().unwrap();

                let new_width = cmp::min(cmp::max(200, x + val),500);
                let new_val = format!("{}{}", new_width.to_string(), "px");
                let _ = style.set_property("flex-basis", &new_val);
                set_sidebar_offset_x.set(ev.screen_x());

            }

        }
        on:mouseup:target=move|ev|{
            if sidebar_dragging.get(){
                set_sidebar_dragging.set(false);
                let target = ev.target().dyn_into::<HtmlElement>().unwrap();
                let children = target.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    if child.class_name() != ".resizer" {
                        let _ = child.dyn_into::<HtmlElement>().unwrap().style().set_property("pointer-events","auto");
                    } 
                }
                let _ = target.style().set_property("cursor", "auto");
            }
        }
        on:mouseleave:target=move|ev|{
            if sidebar_dragging.get(){
                set_sidebar_dragging.set(false);
                let target = ev.target().dyn_into::<HtmlElement>().unwrap();
                let children = target.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    if child.class_name() != ".resizer" {
                        let _ = child.dyn_into::<HtmlElement>().unwrap().style().set_property("pointer-events","auto");
                    } 
                }
                let _ = target.style().set_property("cursor", "auto");
            }
        }
        >
            <div class="sidebar-icons">
                <div class ="temp-buffer"></div>
                <SidebarIcon selected=true id="file-explorer-button".to_string() img_src="public/files.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="account-button".to_string()  img_src="public/account.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="records-button".to_string()  img_src="public/checklist.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="rest-api-button".to_string()  img_src="public/debug-disconnect.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="execute-button".to_string()  img_src="public/play-circle.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="deploy-button".to_string()  img_src="public/cloud-upload.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />

                // <button id ="deploy-button"
                // on:click=move |_|{
                //     let currently_selected = selected_activity_icon.get();
                //     let this_name = "#deploy-button";
                //     if currently_selected == this_name {
                //         let document = leptos::prelude::document();

                //         let details = document.query_selector("#sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                //         let style = details.style();
                        
                //         if style.get_property_value("display").unwrap() == "flex"{
                //             let _ = style.set_property("display", "none");
                //         } else if style.get_property_value("display").unwrap() == "none"{
                //             let _ = style.set_property("display", "flex");
                //         }              
                //     } else {
                //         let document = leptos::prelude::document();

                //         let this = document.query_selector(this_name).unwrap().unwrap();
                //         let currently_selected_element = document.query_selector(currently_selected).unwrap().unwrap();
                //         set_selected_activity_icon.set(this_name);

                //         this.set_class_name("selected");
                //         currently_selected_element.set_class_name("");
                //     }
                // }>
                //     <img src="public/cloud-upload.svg"/>
                // </button>

                <div id ="empty-space"></div>
                <button id ="settings-button">
                    <img src="public/gear.svg"/>
                </button>

            </div>
            <div class="sidebar-details" style="display: flex; flex-basis: 200px;">
                <div class="sidebar-title">File Explorer</div>
                <div class="open-folder-wrapper" style="display:flex;">
                    <button class="open-folder"
                    on:click:target=move|_| {
                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&LoadArgs { code : "null"}).unwrap();

                            let return_val = invoke("open_explorer", args).await.as_string().unwrap();
                            if return_val != ""{
                                let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
                                let fs_html = generate_file_explorer_html(deserialized_return_val);

                                let document = leptos::prelude::document();
                                let element = document.query_selector(".open-folder-wrapper").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element.style().set_property("display", "none");
                                set_fs_html.set(fs_html);
                            }
                        });
                    }
                    > Open Folder </button>
                </div>


                <div class="fs" inner_html={ move || fs_html.get() }></div>
                {Effect::new(move |_| {
                    let _signal = fs_html.get();
                    let document = leptos::prelude::document();
                    let result_element = document.query_selector(".fs").unwrap().unwrap();

                    //Recursively add event listeners to file explorer items
                    let wrapper = RecursiveClosureWrapper {
                        placeholder: result_element.clone(),
                        closure : &|element, closure|{
                            let children = element.children();
                            for i in 0..children.length(){
                                let child = children.get_with_index(i).unwrap();
                                if child.class_name() == "dir" {
                                    let title_element = child.children().named_item("title").unwrap();
                                    let _ = title_element.add_event_listener_with_callback("click", switch_chevron_closure.as_ref().unchecked_ref());
                                    (closure.closure)(child.children().named_item("children").unwrap(), closure);
                                } else if child.class_name() == "file" {
                                    // TODO: ADD EVENT LISTENER FOR OPENING FILES
                                    let title_element = child.children().named_item("title").unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = title_element.add_event_listener_with_callback("dblclick", open_file_closure.as_ref().unchecked_ref());
                                }
                            }
                        }
                    };
                    (wrapper.closure)(result_element,&wrapper);
                });}
            </div>



            <div class="resizer" 
            on:mousedown:target=move|ev|{
                ev.prevent_default();
                set_sidebar_dragging.set(true);
                set_sidebar_offset_x.set(ev.screen_x());

                let document = leptos::prelude::document();
                let result_element = document.query_selector(".main").unwrap().unwrap();
                let children = result_element.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    if child.class_name() != ".resizer" {
                        let _ = child.dyn_into::<HtmlElement>().unwrap().style().set_property("pointer-events","none");
                    } 
                }
                let _ =result_element.dyn_into::<HtmlElement>().unwrap().style().set_property("cursor", "col-resize");
            }
            ></div>




            <div class= "code-terminal-area">
                <div class= "outer-code-area">
                    <div class= "tabs">
                        <For each=move || open_files.get() key=|tuple| tuple.0.clone() children=move |(filepath, filename)| {
                            view! {
                                <FileTab filepath=filepath filename=filename selected_file=selected_file set_selected_file=set_selected_file/>
                            }
                        }/>
                    </div>
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
                </div>
                <div class = "terminal"></div>
            </div>
        </div>
    }
}
