use leptos::ev::Event;
use leptos::web_sys::{Element,HtmlElement, HtmlImageElement, HtmlTextAreaElement, HtmlButtonElement, HtmlInputElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;
use web_sys::css::escape;
use std::cmp;



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
struct LoadFileArgs<> {
    filepath: String
}

#[derive(Serialize, Deserialize)]
struct LoadThemeArgs<'a> {
    code: &'a str,
}

#[derive(Serialize, Deserialize)]
struct WatcherArgs<'a> {
    path: &'a str,
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

// Helper function to get number of lines currently typed in editor
fn get_lines(code2: String) -> String{
    let mut code = code2;
    let previous_index = cmp::max(0, (code.len() as isize)-1) as usize;
    let last_char = &code[previous_index..code.len()];
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

#[component]
fn SidebarFileExplorer (
    open_file_closure: Closure<dyn FnMut(Event)>,
    switch_chevron_closure: Closure<dyn FnMut(Event)>,
    fs_html: ReadSignal<String>,
    set_fs_html : WriteSignal<String>,
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {
    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#file-explorer-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">File Explorer</div>
            <div class="open-folder-wrapper" style="display:flex;">
                <button class="open-folder"
                on:click:target=move|_| {
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&LoadThemeArgs { code : "null"}).unwrap();

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


#[component]
fn SidebarAccount (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("create-new-account-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Create a New Account".to_string());
    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#account-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">Account</div>
            <div id="account-card" class="card">

                <div id="account-dropdown-custom" class="dropdown-custom-head">
                    <div id="account-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                    {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                        if this.class_list().contains("show"){
                            let _ = this.class_list().remove(&new_val);
                            set_dropdown_active.set(false);
                        } else {
                            let _ = this.class_list().add(&new_val);
                            set_dropdown_active.set(true);
                        }
                    }> 
                        <div class="buffer" inner_html={move || current_dropdown_text.get()}></div>
                        <img src="public/chevron-down-dark.svg"/>
                    </div>
                    <div id="account-dropdown-content" class="dropdown-content" style={move || if dropdown_active.get() {"display: block"} else {"display: none"}}>
                        <div id="create-new-account-button" class={move || if current_dropdown_item.get() == "create-new-account-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Create a New Account
                        </div>
                        <div id="load-account-from-pk-button" class={move || if current_dropdown_item.get() == "load-account-from-pk-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Load Account from Private Key
                        </div>
                        <div id="load-address-from-vk-button" class={move || if current_dropdown_item.get() == "load-address-from-vk-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());
                                
                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Load Address from View Key
                        </div>
                        <div id="sign-message-button" class={move || if current_dropdown_item.get() == "sign-message-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Sign a Message
                        </div>
                        <div id="verify-message-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_dropdown_item.get() == "verify-message-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Verify a Message
                        </div>
                    </div>

                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "create-new-account-button" {"display: flex"} else {"display: none"}}>
                    <div id="create-account-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Name</div>
                            <input id="create-account-name-input" placeholder="Account Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="create-account-name-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <img src="public/files.svg"/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="create-account-name-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <img src="public/files.svg"/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="create-account-name-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <img src="public/files.svg"/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="generate-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#create-account-name-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid red");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");   
                        }
                    }
                    >
                        Generate
                    </button>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-account-from-pk-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-account-from-pk-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Private Key</div>
                            <input id="load-account-from-pk-input" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-button" class="card-button"
                    on:click:target=move|_ev| {
                        // let document = leptos::prelude::document();
                        // let current_input = document.query_selector("#create-account-name-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        // let value = current_input.value().clone();
                        // let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        // let style = target.style();
                        // if &value == "" {
                        //     let _ = style.set_property("border", "1px solid red");   
                        // } else {
                        //     let _ = style.set_property("border", "1px solid #494e64");   
                        // }
                    }
                    >
                        Load
                    </button>
                </div>


                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-address-from-vk-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-address-from-vk-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">View Key</div>
                            <input id="load-address-from-vk-input" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-button" class="card-button"
                    on:click:target=move|_ev| {
                        // let document = leptos::prelude::document();
                        // let current_input = document.query_selector("#create-account-name-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        // let value = current_input.value().clone();
                        // let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        // let style = target.style();
                        // if &value == "" {
                        //     let _ = style.set_property("border", "1px solid red");   
                        // } else {
                        //     let _ = style.set_property("border", "1px solid #494e64");   
                        // }
                    }
                    >
                        Load
                    </button>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "sign-message-button" {"display: flex"} else {"display: none"}}>
                    <div id="sign-message-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Private Key</div>
                            <input id="sign-message-input-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field">
                            <div class="field-title">Message</div>
                            <input id="sign-message-input-message" placeholder="Message" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Signature</div>
                            <div class="output-input-wrapper">
                                <input id="sign-message-output" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <img src="public/files.svg"/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <div class="double-button-wrapper" style="order:3; display:flex; justify-content:center">
                        <button id="sign-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {

                        }
                        >
                            Sign
                        </button>
                        <button id="signature-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {

                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "verify-message-button" {"display: flex"} else {"display: none"}}>
                <div id="verify-message-card-body" class="card-body">
                    <div class="input-field">
                        <div class="field-title">Address</div>
                        <input id="verify-message-input-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                    </div>
                    <div class="input-field">
                        <div class="field-title">Message</div>
                        <input id="verify-message-input-message" placeholder="Message" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                    </div>
                    <div class="input-field">
                        <div class="field-title">Signature</div>
                        <input id="verify-message-input-signature" placeholder="Signature" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                    </div>
                </div>
                <div class="card-divider"/>
                    <div class="double-button-wrapper" style="order:3; display:flex; justify-content:center">
                        <button id="verify-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {

                        }
                        >
                            Verify
                        </button>
                        <button id="signature-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {

                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>

            </div>
        </div>
    }
}

#[component]
fn SidebarRecords (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("decrypt-record-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Decrypt Record".to_string());
    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#records-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">Records</div>
            <div id="records-card" style="height: 100%;" class="card">
                <div id="records-dropdown-custom" class="dropdown-custom-head">
                    <div id="records-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                    {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                        if this.class_list().contains("show"){
                            let _ = this.class_list().remove(&new_val);
                            set_dropdown_active.set(false);
                        } else {
                            let _ = this.class_list().add(&new_val);
                            set_dropdown_active.set(true);
                        }
                    }> 
                        <div class="buffer" inner_html={move || current_dropdown_text.get()}></div>
                        <img src="public/chevron-down-dark.svg"/>
                    </div>
                    <div id="records-dropdown-content" class="dropdown-content" style={move || if dropdown_active.get() {"display: block"} else {"display: none"}}>
                        <div id="decrypt-record-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_dropdown_item.get() == "decrypt-record-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#records-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Decrypt Record
                        </div>
                    </div>
                </div>
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "decrypt-record-button" {"display: flex"} else {"display: none"}}>
                    <div id="records-card-body" style="display:flex; flex-direction:column;" class="card-body">
                        <div class="input-field" style="order:0;">
                            <div class="field-title">Name</div>
                            <input id="decrypt-record-input-name" placeholder="Record Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field" style="order:1;">
                            <div class="field-title">Ciphertext</div>
                            <input id="decrypt-record-input-ciphertext" placeholder="Ciphertext" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field" style="order:2;">
                            <div class="field-title">View Key</div>
                            <input id="decrypt-record-input-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="output-field" style="display:flex; flex-direction:column; box-sizing:border-box; height:100%; order:3;">
                            <div style="order:0" class="field-title">Decrypted Record</div>

                            <div class="output-textarea-wrapper">
                                <textarea style="order:0" id="decrypt-record-output" placeholder="Decrypted Record" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-textarea-img-wrapper" style="order:1">
                                    <img src="public/files.svg"/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="decrypt-button" class="card-button"
                    on:click:target=move|_ev| {
                        //TODO: Use SnarkVM to decrypt records using fields
                        let document = leptos::prelude::document();
                        let current_title_input = document.query_selector("#decrypt-record-input-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let current_ciphertext_input = document.query_selector("#decrypt-record-input-ciphertext").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let current_vk_input = document.query_selector("#decrypt-record-input-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                        let title = current_title_input.value().clone();
                        let ciphertext = current_ciphertext_input.value().clone();
                        let vk = current_vk_input.value().clone();


                        let target1 = current_title_input.dyn_into::<HtmlElement>().unwrap();
                        let target2 = current_ciphertext_input.dyn_into::<HtmlElement>().unwrap();
                        let target3 = current_vk_input.dyn_into::<HtmlElement>().unwrap();


                        let style1 = target1.style();
                        let style2 = target2.style();
                        let style3 = target3.style();

                        if &title == "" {
                            let _ = style1.set_property("border", "1px solid red");   
                        } else {
                            let _ = style1.set_property("border", "1px solid #494e64");   
                        }

                        if &ciphertext == "" {
                            let _ = style2.set_property("border", "1px solid red");   
                        } else {
                            let _ = style2.set_property("border", "1px solid #494e64");   
                        }

                        if &vk == "" {
                            let _ = style3.set_property("border", "1px solid red");   
                        } else {
                            let _ = style3.set_property("border", "1px solid #494e64");   
                        }
                    }
                    >
                        Decrypt
                    </button>
                </div>
            </div>
        </div>
    }
}

#[component]
fn SidebarRestApi (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("get-latest-block-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Get Latest Block".to_string());

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#rest-api-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">REST API</div>
            <div id="rest-api-card" class="card">
                <div id="rest-api-dropdown-custom" class="dropdown-custom-head">
                    <div id="rest-api-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                    {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                        if this.class_list().contains("show"){
                            let _ = this.class_list().remove(&new_val);
                            set_dropdown_active.set(false);
                        } else {
                            let _ = this.class_list().add(&new_val);
                            set_dropdown_active.set(true);
                        }
                    }> 
                        <div class="buffer" inner_html={move || current_dropdown_text.get()}></div>
                        <img src="public/chevron-down-dark.svg"/>
                    </div>
                    <div id="rest-api-dropdown-content" class="dropdown-content" style={move || if dropdown_active.get() {"display: block"} else {"display: none"}}>
                        <div id="get-latest-block-button" class={move || if current_dropdown_item.get() == "get-latest-block-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#rest-api-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Get Latest Block
                        </div>
                        <div id="get-block-by-height-button" class={move || if current_dropdown_item.get() == "get-block-by-height-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#rest-api-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Get Block By Height
                        </div>
                        <div id="get-program-button" class={move || if current_dropdown_item.get() == "get-program-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());
                                
                                let document = leptos::prelude::document();
                                let target = document.query_selector("#rest-api-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Get Program
                        </div>
                        <div id="get-transaction-button" class={move || if current_dropdown_item.get() == "get-transaction-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#rest-api-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Get Transaction
                        </div>
                        <div id="get-account-balance-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_dropdown_item.get() == "get-account-balance-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#rest-api-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Get Account Balance
                        </div>
                    </div>

                </div>
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-latest-block-button" {"display: flex"} else {"display: none"}}>
                    <div id="get-latest-block-body" class="card-body"></div>
                    <button id="get-button" class="card-button"
                    on:click:target=move|_ev| {
                    }
                    >
                        Get
                    </button>
                </div>
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-block-by-height-button" {"display: flex"} else {"display: none"}}>
                    <div id="get-block-by-height-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Block Height</div>
                            <input id="get-block-by-height-input" placeholder="Block Height" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="get-button" class="card-button"
                    on:click:target=move|_ev| {
                    }
                    >
                        Get
                    </button>
                </div>  
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-program-button" {"display: flex"} else {"display: none"}}>
                    <div id="get-program-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Program ID</div>
                            <input id="get-program-input" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="get-button" class="card-button"
                    on:click:target=move|_ev| {
                    }
                    >
                        Get
                    </button>
                </div>
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-transaction-button" {"display: flex"} else {"display: none"}}>
                    <div id="get-transaction-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Transaction ID</div>
                            <input id="get-transaction-input" placeholder="Transaction ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="get-button" class="card-button"
                    on:click:target=move|_ev| {
                    }
                    >
                        Get
                    </button>
                </div>
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-account-balance-button" {"display: flex"} else {"display: none"}}>
                    <div id="get-account-balance-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Address</div>
                            <input id="get-account-balance-input" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="get-button" class="card-button"
                    on:click:target=move|_ev| {
                    }
                    >
                        Get
                    </button>
                </div>                           
            </div>
        </div>
    }
}

#[component]
fn SidebarDeployExecute (
    selected_activity_icon: ReadSignal<String>,
    // accounts: ReadSignal<Vec<String>>
) -> impl IntoView {
    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("deploy-new-program-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Deploy a New Program".to_string());

    let (environment_dropdown_active, set_environment_dropdown_active) = signal(false);
    let (current_environment_dropdown_item, set_current_environment_dropdown_item) = signal("devnet-button".to_string());
    let (current_environment_dropdown_text, set_current_environment_dropdown_text) = signal("Local Devnet".to_string());

    let (environment_dropdown_active2, set_environment_dropdown_active2) = signal(false);
    let (current_environment_dropdown_item2, set_current_environment_dropdown_item2) = signal("".to_string());
    let (current_environment_dropdown_text2, set_current_environment_dropdown_text2) = signal("--".to_string());

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#execute-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">
                Deploy and Execute
            </div>

            <div id="environment-card" style="color:#e3e3e3;" class="card">
                <div id="environment-card-head" class="card-head" >
                    <div class="title" style="-webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;">
                        Environment
                    </div>
                </div>
                <div class="card-body-wrapper">
                    <div id="deploy-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Network</div>
                            <div id="environment-dropdown-custom" class="dropdown-custom">
                                <div id="environment-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                                {
                                    let this = ev.target().dyn_into::<Element>().unwrap();
                                    let new_val = Array::new();
                                    new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                    if this.class_list().contains("show"){
                                        let _ = this.class_list().remove(&new_val);
                                        set_environment_dropdown_active.set(false);
                                    } else {
                                        let _ = this.class_list().add(&new_val);
                                        set_environment_dropdown_active.set(true);
                                    }
                                }> 
                                    <div class="buffer" inner_html={move || current_environment_dropdown_text.get()}></div>
                                    <img src="public/chevron-down.svg"/>
                                </div>
                                <div id="environment-dropdown-content" class="dropdown-content" style={move || if environment_dropdown_active.get() {"display: block"} else {"display: none"}}>
                                    <div id="devnet-button" class={move || if current_environment_dropdown_item.get() == "devnet-button" {"dropdown-item selected"} else {"dropdown-item"}}
                                    on:click:target = move|ev| {
                                        if current_environment_dropdown_item.get() != ev.target().id(){
                                            set_current_environment_dropdown_item.set(ev.target().id());
                                            set_current_environment_dropdown_text.set(ev.target().inner_html());

                                            let document = leptos::prelude::document();
                                            let target = document.query_selector("#environment-dropdown-button").unwrap().unwrap();
                                            let new_val = Array::new();
                                            new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                            let _ = target.class_list().remove(&new_val);
                                            set_environment_dropdown_active.set(false);
                                        }
                                    }
                                    >
                                        Local Devnet
                                    </div>
                                    <div id="testnet-button" class={move || if current_environment_dropdown_item.get() == "testnet-button" {"dropdown-item selected"} else {"dropdown-item"}}
                                    on:click:target = move|ev| {
                                        if current_environment_dropdown_item.get() != ev.target().id(){
                                            set_current_environment_dropdown_item.set(ev.target().id());
                                            set_current_environment_dropdown_text.set(ev.target().inner_html());

                                            let document = leptos::prelude::document();
                                            let target = document.query_selector("#environment-dropdown-button").unwrap().unwrap();
                                            let new_val = Array::new();
                                            new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                            let _ = target.class_list().remove(&new_val);
                                            set_environment_dropdown_active.set(false);
                                        }
                                    }
                                    >
                                        Testnet
                                    </div>

                                    <div id="mainnet-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_environment_dropdown_item.get() == "mainnet-button" {"dropdown-item selected"} else {"dropdown-item"}}
                                    on:click:target = move|ev| {
                                        if current_environment_dropdown_item.get() != ev.target().id(){
                                            set_current_environment_dropdown_item.set(ev.target().id());
                                            set_current_environment_dropdown_text.set(ev.target().inner_html());

                                            let document = leptos::prelude::document();
                                            let target = document.query_selector("#environment-dropdown-button").unwrap().unwrap();
                                            let new_val = Array::new();
                                            new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                            let _ = target.class_list().remove(&new_val);
                                            set_environment_dropdown_active.set(false);
                                        }
                                    }
                                    >
                                        Mainnet
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="input-field">
                            <div class="field-title">Account</div>
                            <div id="environment-dropdown-custom2" class="dropdown-custom">
                                <div id="environment-dropdown-button2" class="dropdown-button" on:click:target=move|ev| 
                                {
                                    let this = ev.target().dyn_into::<Element>().unwrap();
                                    let new_val = Array::new();
                                    new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                    if this.class_list().contains("show"){
                                        let _ = this.class_list().remove(&new_val);
                                        set_environment_dropdown_active2.set(false);
                                    } else {
                                        let _ = this.class_list().add(&new_val);
                                        set_environment_dropdown_active2.set(true);
                                    }
                                }> 
                                    <div class="buffer" inner_html={move || current_environment_dropdown_text2.get()}></div>
                                    <img src="public/chevron-down.svg"/>
                                </div>
                                <div id="environment-dropdown-content2" class="dropdown-content" style={move || if environment_dropdown_active2.get() {"display: block"} else {"display: none"}}>
                                    <div id="placeholder-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if false {"dropdown-item-placeholder selected"} else {"dropdown-item-placeholder"}}
                                    >
                                        Please load an account first!
                                    </div>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>

            <div class="panel-divider"/>
            
            <div id="deploy-and-execute-card" class="card">
                <div id="deploy-and-execute-dropdown-custom" class="dropdown-custom-head">
                    <div id="deploy-and-execute-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                    {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                        if this.class_list().contains("show"){
                            let _ = this.class_list().remove(&new_val);
                            set_dropdown_active.set(false);
                        } else {
                            let _ = this.class_list().add(&new_val);
                            set_dropdown_active.set(true);
                        }
                    }> 
                        <div class="buffer" inner_html={move || current_dropdown_text.get()}></div>
                        <img src="public/chevron-down-dark.svg"/>
                    </div>
                    <div id="deploy-and-execute-dropdown-content" class="dropdown-content" style={move || if dropdown_active.get() {"display: block"} else {"display: none"}}>
                        <div id="deploy-new-program-button" class={move || if current_dropdown_item.get() == "deploy-new-program-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#deploy-and-execute-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Deploy a New Program
                        </div>
                        <div id="load-program-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_dropdown_item.get() == "load-program-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#deploy-and-execute-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Load Existing Program
                        </div>
                    </div>
                </div>





                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "deploy-new-program-button" {"display: flex"} else {"display: none"}}>
                    <div id="deploy-program-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Program ID</div>
                            <input id="deploy-input-program-id" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field">
                            <div class="field-title">Private Key</div>
                            <input id="deploy-input-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>

                        <div class="input-field">
                            <div class="field-title">Fee</div>
                            <div class="output-input-wrapper">
                                <input id="deploy-input-fee" style=" border-top-right-radius: 0px; border-bottom-right-radius: 0px;" placeholder="Fee" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div class="card-button-estimate-fee">Estimate Fee</div>
                            </div>
                        </div>

                        <div class="switch-wrapper">
                        <div class="field-title" style="order:0; padding:0; margin-right:15px; padding-top:2.5px; padding-bottom:2.5px;">Private Fee</div>
                            <label class="switch" style="order:1;">
                                <input type="checkbox"/>
                                <span class="slider round"></span>
                            </label>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="deploy-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#load-program-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid red");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");   
                        }
                    }
                    >
                        Deploy
                    </button>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-program-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-program-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Program ID</div>
                            <input id="load-program-input" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-program-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#load-program-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid red");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");   
                        }
                    }
                    >
                        Load
                    </button>
                </div>
            </div>

            <div class="panel-divider"/>

        </div>
    }
}

// #[component]
// fn SidebarDeploy (
//     selected_activity_icon: ReadSignal<String>
// ) -> impl IntoView {
//     view! {
//         <div class="wrapper" style={move || if selected_activity_icon.get() == "#deploy-button" {"display: flex;"} else {"display: none;"}}>
//             <div class="sidebar-title"> Deploy </div>
//         </div>
//     }
// }





#[component]
fn FileTab(
    filepath: String,
    filename: String,
    selected_file: ReadSignal<String>,
    set_selected_file : WriteSignal<String>,
    open_files : ReadSignal<Vec<(String,String)>>,
    set_open_files : WriteSignal<Vec<(String,String)>>
) -> impl IntoView {
    view! {
        <div class = "tab" id=filepath.clone()
        on:click:target = move|ev|{
            let target = ev.target().dyn_into::<Element>().unwrap();
            let new_val = Array::new();
            new_val.push(&serde_wasm_bindgen::to_value("selected").unwrap());
            let _ = target.class_list().add(&new_val);
            set_selected_file.set(target.id());
        }>
            {filename.clone()}
            <button class="exit-button"
            on:click:target = {
                let filepath_clone = filepath.clone();
                let filename_clone = filename.clone();
                move|ev|{
                ev.stop_propagation();
                for index in 0..open_files.get().len(){
                    let mut vec = open_files.get();
                    if vec[index] == ((&filepath_clone).to_string(),(&filename_clone).to_string()){
                        vec.remove(index);
                        if vec.len() > 0 {
                            if index == vec.len(){
                                set_selected_file.set(vec[index-1].0.clone());
                            } else if index < vec.len() {
                                set_selected_file.set(vec[index].0.clone());
                            }
                        } else if vec.len() == 0 {
                            set_selected_file.set(String::new());
                        }
                        set_open_files.set(vec);
                        break;
                    }
                }
                }
            }>
                <img src="public/close.svg"/>
            </button>
        </div>

        {Effect::new(move |_| {
            let document = leptos::prelude::document();
            let target_string = format!("{}{}", "#", escape(&filepath));
            let target2 = document.query_selector(&target_string).unwrap();
            match target2 {
                Some(e) => {
                    let target = e;
                    let new_val = Array::new();
                    new_val.push(&serde_wasm_bindgen::to_value("selected").unwrap());
                    if selected_file.get() != target.id(){
                        let _ = target.class_list().remove(&new_val);
                    } else if selected_file.get() == target.id(){
                        let _ = target.class_list().add(&new_val);
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

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    // let (test, set_test) = signal(String::new());


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
    let (open_files, set_open_files) = signal(new_vec.clone());


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


    /*
    ==============================================================================
    EVENT LISTENERS
    ==============================================================================
    */


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
        set_open_files.update(|vec| if !vec.contains(&(filepath.clone(),filename.clone())){vec.push((filepath.clone(),filename.clone()))});
        
    }) as Box<dyn FnMut(_)>);



    /*
    ==============================================================================
    STARTUP EFFECTS
    ==============================================================================
    */


    // Load Syntax and Color Scheme from file
    Effect::new(move |_| {
        spawn_local(async move {
            let args = serde_wasm_bindgen::to_value(&LoadThemeArgs { code : "null"}).unwrap();
            let return_tuple: (SyntaxSet, Theme) = serde_wasm_bindgen::from_value(invoke("load", args).await).unwrap();
            set_syntax_set.set(return_tuple.0);
            set_theme.set(return_tuple.1);
        });
    });

    
    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

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

                let new_width = cmp::min(cmp::max(300, x + val),500);
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
                // <SidebarIcon id="deploy-button".to_string()  img_src="public/cloud-upload.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />

                <div id ="empty-space"></div>
                <button id ="settings-button">
                    <img src="public/gear.svg"/>
                </button>

            </div>
            <div class="sidebar-details" style="display: flex; flex-basis: 300px;">
                // <div style="color:red" inner_html={ move || test.get() }/>
                <SidebarFileExplorer open_file_closure=open_file_closure switch_chevron_closure=switch_chevron_closure fs_html=fs_html set_fs_html=set_fs_html selected_activity_icon=selected_activity_icon />
                <SidebarAccount selected_activity_icon=selected_activity_icon/>
                <SidebarRecords selected_activity_icon=selected_activity_icon/>
                <SidebarRestApi selected_activity_icon=selected_activity_icon/>
                <SidebarDeployExecute selected_activity_icon=selected_activity_icon/>
                // <SidebarDeploy selected_activity_icon=selected_activity_icon/>
                
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
                                <FileTab filepath=filepath filename=filename selected_file=selected_file set_selected_file=set_selected_file open_files=open_files set_open_files=set_open_files/>
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
                                let lines_html = get_lines(ev.target().value());
                                set_lines_html.set(lines_html);

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


                            {Effect::new(move |_| {
                                let selected = selected_file.get();
                                if selected != String::new(){
                                    spawn_local(
                                        async move {
                                            let args = serde_wasm_bindgen::to_value(&LoadFileArgs{filepath: selected}).unwrap();
                                            match invoke("read_file", args).await.as_string(){
                                                Some(contents) => {
                                                    let args = serde_wasm_bindgen::to_value(&HighlightArgs { code: &contents, ss : syntax_set.get_untracked(), theme : theme.get_untracked()}).unwrap();
                                                    let highlighted = invoke("highlight", args).await.as_string().unwrap();
                                                    set_highlighted_msg.set(highlighted);
        
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
                                            
                                            //.as_string().unwrap();
                                            

                                        }
                                    );
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
                </div>
                <div class = "terminal"></div>
            </div>
        </div>
    }
}
