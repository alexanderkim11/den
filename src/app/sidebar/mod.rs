pub mod file_explorer;
pub use file_explorer::*;

pub mod account;
pub use account::*;

pub mod records;
pub use records::*;

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