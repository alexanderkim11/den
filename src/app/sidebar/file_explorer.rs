use leptos::ev::Event;
use leptos::web_sys::{Element,HtmlElement, HtmlImageElement, HtmlTextAreaElement};
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
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
pub struct PlaceholderArgs<'a> {
    pub code: &'a str,
}

#[derive(Serialize, Deserialize)]
pub struct CustomDirEntry<> {
    pub path: String,
    pub name: String,
    pub type_of: String,
    pub subpaths : Vec<CustomDirEntry<>>,
}

pub struct RecursiveClosureWrapper<'s>  {
    placeholder: Element,
    pub closure : &'s dyn Fn(Element,&RecursiveClosureWrapper)
}


/*
==============================================================================
HELPER FUNCTIONS
==============================================================================
*/

// Recursively generate file explorer html
pub fn generate_file_explorer_html(mut dir_entry: Vec<CustomDirEntry>) -> String {
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
            let extension = entry.path.split(".").collect::<Vec<&str>>();
            if extension[extension.len()-1] == "leo"{
                fs_html = format!("{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/leo.svg\" style=\" padding-left:2px; padding-top:1px;  padding-bottom:1px;  width:16px; height:15px;\"/><div style=\"padding-left:3.5px\">", entry.name, "</div></div></div>");
            } else if extension[extension.len()-1] == "aleo"{
                fs_html = format!("{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/aleo2.svg\" style=\"width:12px; height:13px; padding-top:2px; padding-left:5px; padding-bottom:2px; padding-right:2px;\"/><div>", entry.name, "</div></div></div>");
            } else {
                fs_html = format!("{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/file.svg\"/><div>", entry.name, "</div></div></div>");

            }
        }
    }
    return fs_html;

}




#[component]
pub fn SidebarFileExplorer (
    fs_html: ReadSignal<String>,
    set_fs_html : WriteSignal<String>,
    selected_activity_icon: ReadSignal<String>,
    selected_file : ReadSignal<String>,
    set_selected_file: WriteSignal<String>,
    set_open_files: WriteSignal<Vec<(String,String)>>,
    saved_file_contents: ReadSignal<HashMap<String,String>>, 
    set_saved_file_contents: WriteSignal<HashMap<String,String>>,
    cached_file_contents: ReadSignal<HashMap<String,String>>, 
    set_cached_file_contents: WriteSignal<HashMap<String,String>>
) -> impl IntoView {


    /*
    ==============================================================================
    EVENT LISTENERS
    ==============================================================================
    */


    //Used for opening files
    let open_file_closure = Closure::wrap(Box::new(move |ev: Event| {
        let title_element = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        let dataset = title_element.dataset();
        let filepath = dataset.get("filepath").unwrap();
        let temp = filepath.clone();
        let collection = temp.split("\\").collect::<Vec<&str>>();
        let filename = collection[collection.len()-1].to_string();


        let current_filepath = selected_file.get_untracked();
        let mut cached_content = cached_file_contents.get_untracked();

        let document = leptos::prelude::document();
        let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

        cached_content.remove(&current_filepath);
        cached_content.insert(current_filepath,result_element.value());
        set_cached_file_contents.set(cached_content);

        set_selected_file.set(filepath.clone());
        set_open_files.update(|vec| if !vec.contains(&(filepath.clone(),filename.clone())){vec.push((filepath.clone(),filename.clone()))});
        
    }) as Box<dyn FnMut(_)>);


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
    

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#file-explorer-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">File Explorer</div>
            <div class="open-folder-wrapper" style="display:flex;">
                <button class="open-folder"
                on:click:target=move|_| {
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { code : "null"}).unwrap();

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
                                let title_element = child.children().named_item("title").unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = title_element.add_event_listener_with_callback("dblclick", open_file_closure.as_ref().unchecked_ref());
                            }
                        }
                    }
                };
                (wrapper.closure)(result_element,&wrapper);
            });}
        </div>
    }
}
