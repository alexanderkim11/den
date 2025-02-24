pub mod file_explorer;
pub use file_explorer::*;

pub mod environment;
pub use environment::*;

pub mod account;
pub use account::*;

pub mod records;
pub use records::*;

pub mod compile;
pub use compile::*;

pub mod deploy_execute;
pub use deploy_execute::*;

pub mod rest_api;
pub use rest_api::*;


use leptos::web_sys::{HtmlElement,HtmlInputElement, HtmlTextAreaElement};
use leptos::task::spawn_local;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};


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
pub struct CopyArgs<> {
    pub value : String
}

#[derive(Serialize, Deserialize)]
pub struct CustomDirEntry<> {
    pub path: String,
    pub name: String,
    pub type_of: String,
    pub subpaths : Vec<CustomDirEntry<>>,
}


/*
==============================================================================
FUNCTIONS
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
            let fs_title = format!("{}{}{}{}{}{}{}", "<div name = \"title\" class=\"fs-title\" id=\"fs_",entry.path,"\"><img name = \"image\" id=\"", entry.path, "--img\" class=\"inactive\" src=\"public/chevron-right.svg\"/><div>", entry.name, "</div></div>");
            let dir_children = format!("{}{}{}{}{}", "<div name = \"children\" id=\"fs_",entry.path,"--children\" class=\"dir-children\">", generate_file_explorer_html(subpaths), "</div>");
            fs_html = format!("{}{}{}{}{}", fs_html,"<div class=\"dir\">", fs_title, dir_children, "</div>");

        } else if entry_type == "File" {
            let extension = entry.path.split(".").collect::<Vec<&str>>();
            if extension[extension.len()-1] == "leo"{
                fs_html = format!("{}{}{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" id=\"fs_",entry.path,"\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/leo.svg\" style=\" padding-left:2px; padding-top:1px;  padding-bottom:1px;  width:16px; height:15px;\"/><div style=\"padding-left:3.5px\">", entry.name, "</div></div></div>");
            } else if extension[extension.len()-1] == "aleo"{
                fs_html = format!("{}{}{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" id=\"fs_",entry.path,"\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/aleo2.svg\" style=\"width:12px; height:13px; padding-top:2px; padding-left:5px; padding-bottom:2px; padding-right:2px;\"/><div>", entry.name, "</div></div></div>");
            } else {
                fs_html = format!("{}{}{}{}{}{}{}{}",fs_html,"<div class=\"file\"> <div name = \"title\" id=\"fs_",entry.path,"\" data-filepath=\"", entry.path, "\" class=\"fs-title\"><img src=\"public/file.svg\"/><div>", entry.name, "</div></div></div>");

            }
        }
    }
    return fs_html;

}



/*
==============================================================================
COMPONENTS
==============================================================================
*/
#[component]
pub fn CopyButton(
    target_field : String,
    element_type : String
) -> impl IntoView {
    view!{
        <img src="public/files.svg" style="-webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;"
        on:click:target= move |_| {
            let document = leptos::prelude::document();
            if element_type == "Input".to_string(){
                let current_input = document.query_selector(&target_field).unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                let value = current_input.value().clone();
                if &value != "" {
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&CopyArgs{value: value}).unwrap();
                        invoke("copy",args).await;
                    });
                }
            } else if element_type == "TextArea" {
                let current_input = document.query_selector(&target_field).unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                let value = current_input.value().clone();
                if &value != "" {
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&CopyArgs{value: value}).unwrap();
                        invoke("copy",args).await;
                    });
                }
            }
        }/>
    }
}


#[component]
pub fn SidebarIcon(
    #[prop(optional)]
    selected: bool,
    #[prop(optional)]
    style:String,
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
        <button id = id.clone() class=class style=style
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

                let details = document.query_selector(".sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                let style = details.style();
                if style.get_property_value("display").unwrap() == "none"{
                    let _ = style.set_property("display", "flex");
                }
            }
        }>
            <img src=img_src/>
        </button>
    }
}