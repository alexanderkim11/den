use leptos::ev::Event;
use leptos::web_sys::{Element,HtmlElement,HtmlInputElement, HtmlImageElement, HtmlTextAreaElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;



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
pub struct Command<> {
    pub command : Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct CopyArgs<> {
    pub value : String
}

#[derive(Serialize, Deserialize)]
pub struct DecryptRecordArgs<> {
    pub network : String,
    pub ciphertext: String,
    pub viewkey : String
}

#[derive(Serialize, Deserialize)]
pub struct AddressFromViewKeyArgs<> {
    pub network : String,
    pub viewkey : String
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
        <img src="public/files.svg" 
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

#[component]
pub fn SidebarFileExplorer (
    fs_html: ReadSignal<String>,
    set_fs_html : WriteSignal<String>,
    selected_activity_icon: ReadSignal<String>,
    set_selected_file: WriteSignal<String>,
    set_open_files: WriteSignal<Vec<(String,String)>>
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


#[component]
pub fn SidebarAccount (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("create-new-account-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Create a New Account".to_string());

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

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
                            <input id="create-new-account-input-name" placeholder="Account Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="create-new-account-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#create-new-account-output-pk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="create-new-account-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#create-new-account-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="create-new-account-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#create-new-account-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="create-new-account-generate-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();

                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&Command { command : vec!["account".to_string(),"new".to_string()]}).unwrap();
    
                            let output: (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                            let mut results : [&str; 3] = [""; 3];
                            if output.0 {
                                let split = output.1.split("\n\n").collect::<Vec<&str>>();
                                let trimmed_split = &(split[1..split.len()-2]);
                                for i in 0..trimmed_split.len(){
                                    let split2 = trimmed_split[i].trim_start().split("  ").collect::<Vec<&str>>();
                                    results[i] = split2[1]
                                }
                                let pk = results[0];
                                let vk = results[1];
                                let address = results[2];

                                let pk_output_element = document.query_selector("#create-new-account-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let vk_output_element = document.query_selector("#create-new-account-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let address_output_element = document.query_selector("#create-new-account-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                                pk_output_element.set_value(pk);
                                vk_output_element.set_value(vk);
                                address_output_element.set_value(address);

                                let old_button = document.query_selector("#create-new-account-generate-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let new_button = document.query_selector("#create-new-account-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_button.style().set_property("display", "none");
                                let _ = new_button.style().set_property("display", "flex");    

                            } else {

                            }
                        });
                    }
                    >
                        Generate
                    </button>
                    <div id="create-new-account-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                        <button id="create-new-account-save-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#create-new-account-input-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = current_input.value().clone();
                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            if &value == "" {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style.set_property("border", "1px solid #494e64"); 
                                let pk_output_element = document.query_selector("#create-new-account-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let vk_output_element = document.query_selector("#create-new-account-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let address_output_element = document.query_selector("#create-new-account-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        
                                // let pk = current_address_output.value();
                                // let message = current_message_output.value();
                                // let signature = current_signature_output.value();
                            }
                        }
                        >
                            Save
                        </button>
                        <button id="create-new-account-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();

                            let input_style = document.query_selector("#create-new-account-input-name").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap().style();
                            let _ = input_style.set_property("border", "1px solid #494e64"); 

                            let new_button = document.query_selector("#create-new-account-generate-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_button = document.query_selector("#create-new-account-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            
                            let _ = old_button.style().set_property("display", "none");
                            let _ = new_button.style().set_property("display", "inline-block");    

                            let pk_output_element = document.query_selector("#create-new-account-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_output_element = document.query_selector("#create-new-account-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let address_output_element = document.query_selector("#create-new-account-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            
                            pk_output_element.set_value("");
                            vk_output_element.set_value("");
                            address_output_element.set_value("");
                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-account-from-pk-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-account-from-pk-input-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Private Key</div>
                            <input id="load-account-from-pk-input" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="load-account-from-pk-input-error" class="error-title" style="display:none;"></div>
                        </div>
                    </div>
                    <div id="load-account-from-pk-output-card-body" class="card-body" style="display:none;">
                        <div class="input-field">
                            <div class="field-title">Name</div>
                            <input id="load-account-from-pk-output-name" placeholder="Account Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="load-account-from-pk-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-account-from-pk-output-pk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="load-account-from-pk-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-account-from-pk-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="load-account-from-pk-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-account-from-pk-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-account-from-pk-load-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#load-account-from-pk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = (&current_input).value();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");
                            let error = document.query_selector("#load-account-from-pk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error.style().set_property("display", "none");

                            spawn_local(async move {
                                let args = serde_wasm_bindgen::to_value(&Command { command : vec!["account".to_string(),"import".to_string(), value]}).unwrap();

                                //Reset pk input so it doesn't get remain
                                let current_input = document.query_selector("#load-account-from-pk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                current_input.set_value("");

                                let output: (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                let mut results : [&str; 3] = [""; 3];
                                if output.0 {
                                    let split = output.1.split("\n\n").collect::<Vec<&str>>();
                                    let trimmed_split = &(split[1..split.len()-2]);
                                    for i in 0..trimmed_split.len(){
                                        let split2 = trimmed_split[i].trim_start().split("  ").collect::<Vec<&str>>();
                                        results[i] = split2[1]
                                    }
                                    let pk = results[0];
                                    let vk = results[1];
                                    let address = results[2];
    
                                    let pk_output_element = document.query_selector("#load-account-from-pk-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let vk_output_element = document.query_selector("#load-account-from-pk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let address_output_element = document.query_selector("#load-account-from-pk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                                    pk_output_element.set_value(pk);
                                    vk_output_element.set_value(vk);
                                    address_output_element.set_value(address);

                                    let old_body = document.query_selector("#load-account-from-pk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_body = document.query_selector("#load-account-from-pk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let old_button = document.query_selector("#load-account-from-pk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_button = document.query_selector("#load-account-from-pk-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    
                                    let _ = old_body.style().set_property("display", "none");
                                    let _ = old_button.style().set_property("display", "none");
                                    let _ = new_body.style().set_property("display", "block");
                                    let _ = new_button.style().set_property("display", "flex");          
    
                                } else {
                                    let error = document.query_selector("#load-account-from-pk-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    error.set_inner_html("Error: Invalid private key");
                                    let _ = error.style().set_property("display", "block");
                                }
                            });   
                        }
                    }
                    >
                        Load
                    </button>
                    <div id="load-account-from-pk-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                        <button id="load-account-from-pk-save-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();
                            let current_name_input = document.query_selector("#load-account-from-pk-output-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let name = current_name_input.value();
                            let target = current_name_input.dyn_into::<HtmlElement>().unwrap();
                            if name == "".to_string() {
                                let _ = target.style().set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = target.style().set_property("border", "1px solid #494e64");;   
                                let pk_output_element = document.query_selector("#load-account-from-pk-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let vk_output_element = document.query_selector("#load-account-from-pk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let address_output_element = document.query_selector("#load-account-from-pk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        
                                // let pk = current_address_output.value();
                                // let message = current_message_output.value();
                                // let signature = current_signature_output.value();
                            }
                        }
                        >
                            Save
                        </button>
                        <button id="load-account-from-pk-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();

                            let new_body = document.query_selector("#load-account-from-pk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_body = document.query_selector("#load-account-from-pk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let new_button = document.query_selector("#load-account-from-pk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_button = document.query_selector("#load-account-from-pk-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            
                            let _ = old_body.style().set_property("display", "none");
                            let _ = old_button.style().set_property("display", "none");
                            let _ = new_body.style().set_property("display", "block");
                            let _ = new_button.style().set_property("display", "inline-block");    

                            let pk_output_element = document.query_selector("#load-account-from-pk-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_output_element = document.query_selector("#load-account-from-pk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let address_output_element = document.query_selector("#load-account-from-pk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            
                            pk_output_element.set_value("");
                            vk_output_element.set_value("");
                            address_output_element.set_value("");
                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>


                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-address-from-vk-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-address-from-vk-input-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">View Key</div>
                            <input id="load-address-from-vk-input" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="load-address-from-vk-input-error" class="error-title" style="display:none;"></div>
                        </div>
                    </div>

                    <div id="load-address-from-vk-output-card-body" class="card-body" style="display:none;">
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="load-address-from-vk-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-address-from-vk-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="load-address-from-vk-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-address-from-vk-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-address-from-vk-load-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#load-address-from-vk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = (&current_input).value();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");
                            let error = document.query_selector("#load-address-from-vk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error.style().set_property("display", "none");
    
                            spawn_local(async move {
                                let args = serde_wasm_bindgen::to_value(&AddressFromViewKeyArgs {network : "Mainnet".to_string(), viewkey : value.clone()}).unwrap();
                                let (error, address): (bool, String) = serde_wasm_bindgen::from_value(invoke("address_from_vk", args).await).unwrap();
                                
                                if !error{
                                    //Reset vk input so it doesn't get remain
                                    let current_input = document.query_selector("#load-address-from-vk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    current_input.set_value("");

                                    let vk_output_element = document.query_selector("#load-address-from-vk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let address_output_element = document.query_selector("#load-address-from-vk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                                    vk_output_element.set_value(&value);
                                    address_output_element.set_value(&address);

                                    let old_body = document.query_selector("#load-address-from-vk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_body = document.query_selector("#load-address-from-vk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let old_button = document.query_selector("#load-address-from-vk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_button = document.query_selector("#load-address-from-vk-cancel-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    
                                    let _ = old_body.style().set_property("display", "none");
                                    let _ = old_button.style().set_property("display", "none");
                                    let _ = new_body.style().set_property("display", "block");
                                    let _ = new_button.style().set_property("display", "inline-block");          
    
                                } else {
                                    let error = document.query_selector("#load-address-from-vk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    error.set_inner_html("Error: Invalid private key");
                                    let _ = error.style().set_property("display", "block");
                                }
                            });   
                        }
                    }
                    >
                        Load
                    </button>
                    <button id="load-address-from-vk-cancel-button" class="card-button-clear" style="display:none;"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
    
                        let new_body = document.query_selector("#load-address-from-vk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let old_body = document.query_selector("#load-address-from-vk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_button = document.query_selector("#load-address-from-vk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let old_button = document.query_selector("#load-address-from-vk-cancel-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        
                        let _ = old_body.style().set_property("display", "none");
                        let _ = old_button.style().set_property("display", "none");
                        let _ = new_body.style().set_property("display", "block");
                        let _ = new_button.style().set_property("display", "inline-block");    
    
                        let vk_output_element = document.query_selector("#load-address-from-vk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let address_output_element = document.query_selector("#load-address-from-vk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        
                        vk_output_element.set_value("");
                        address_output_element.set_value("");
                    }
                    >
                        Cancel
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
                                    <CopyButton target_field="#sign-message-output".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <div class="double-button-wrapper" style="order:3; display:flex; justify-content:center">
                        <button id="sign-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();
                            
                            let current_pk_input = document.query_selector("#sign-message-input-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_message_input = document.query_selector("#sign-message-input-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                            let pk = current_pk_input.value().clone();
                            let message = current_message_input.value().clone();
    
                            let target1 = current_pk_input.dyn_into::<HtmlElement>().unwrap();
                            let target2 = current_message_input.dyn_into::<HtmlElement>().unwrap();
    
                            let style1 = target1.style();
                            let style2 = target2.style();
    
                            if &pk == "" {
                                let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style1.set_property("border", "1px solid #494e64");   
                            }
    
                            if &message == "" {
                                let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style2.set_property("border", "1px solid #494e64");   
                            }
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
                            let document = leptos::prelude::document();

                            let current_address_input = document.query_selector("#verify-message-input-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_message_input = document.query_selector("#verify-message-input-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_signature_input = document.query_selector("#verify-message-input-signature").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                            let address = current_address_input.value().clone();
                            let message = current_message_input.value().clone();
                            let signature = current_signature_input.value().clone();
    
    
                            let target1 = current_address_input.dyn_into::<HtmlElement>().unwrap();
                            let target2 = current_message_input.dyn_into::<HtmlElement>().unwrap();
                            let target3 = current_signature_input.dyn_into::<HtmlElement>().unwrap();
    
    
                            let style1 = target1.style();
                            let style2 = target2.style();
                            let style3 = target3.style();
    
                            if &address == "" {
                                let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style1.set_property("border", "1px solid #494e64");   
                            }
    
                            if &message == "" {
                                let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style2.set_property("border", "1px solid #494e64");   
                            }
    
                            if &signature == "" {
                                let _ = style3.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style3.set_property("border", "1px solid #494e64");   
                            }
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
pub fn SidebarRecords (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("decrypt-record-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Decrypt Record".to_string());

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

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
                                <textarea style="order:0; white-space: pre-wrap;" id="decrypt-record-output" placeholder="Decrypted Record" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-textarea-img-wrapper" style="order:1">
                                    <CopyButton target_field="#decrypt-record-output".to_string() element_type="TextArea".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="decrypt-button" class="card-button"
                    on:click:target=move|_ev| {
                        //TODO: Use SnarkVM to decrypt records using input values
                        let document = leptos::prelude::document();
                        let name_input_element = document.query_selector("#decrypt-record-input-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let ciphertext_input_element = document.query_selector("#decrypt-record-input-ciphertext").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let vk_input_element = document.query_selector("#decrypt-record-input-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                        let name = name_input_element.value();
                        let ciphertext = ciphertext_input_element.value();
                        let vk = vk_input_element.value();


                        let target1 = name_input_element.dyn_into::<HtmlElement>().unwrap();
                        let target2 = ciphertext_input_element.dyn_into::<HtmlElement>().unwrap();
                        let target3 = vk_input_element.dyn_into::<HtmlElement>().unwrap();


                        let style1 = target1.style();
                        let style2 = target2.style();
                        let style3 = target3.style();

                        if &name == "" {
                            let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style1.set_property("border", "1px solid #494e64");   
                        }

                        if &ciphertext == "" {
                            let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style2.set_property("border", "1px solid #494e64");   
                        }

                        if &vk == "" {
                            let _ = style3.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style3.set_property("border", "1px solid #494e64");   
                        }
                        
                        if &ciphertext != "" && &vk != ""{
                            let _ = style2.set_property("border", "1px solid #494e64");   
                            let _ = style3.set_property("border", "1px solid #494e64");  
                            spawn_local(async move{
                                let args = serde_wasm_bindgen::to_value(&DecryptRecordArgs{network: "Mainnet".to_string(), ciphertext : ciphertext, viewkey : vk}).unwrap();
                                let (error,plaintext) : (bool, String) = serde_wasm_bindgen::from_value(invoke("decrypt_record", args).await).unwrap();
                                if !error {
                                    let output_element = document.query_selector("#decrypt-record-output").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                    output_element.set_inner_html(&plaintext); 
                                } else {

                                }

                            });
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
pub fn SidebarDeployExecute (
    selected_activity_icon: ReadSignal<String>,
    // accounts: ReadSignal<Vec<String>>
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("deploy-new-program-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Deploy a New Program".to_string());

    let (environment_dropdown_active, set_environment_dropdown_active) = signal(false);
    let (current_environment_dropdown_item, set_current_environment_dropdown_item) = signal("devnet-button".to_string());
    let (current_environment_dropdown_text, set_current_environment_dropdown_text) = signal("Local Devnet".to_string());

    let (environment_dropdown_active2, set_environment_dropdown_active2) = signal(false);
    let (current_environment_dropdown_item2, set_current_environment_dropdown_item2) = signal("".to_string());
    let (current_environment_dropdown_text2, set_current_environment_dropdown_text2) = signal("--".to_string());

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#deploy-execute-button" {"display: flex;"} else {"display: none;"}}>
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
                                                    
                        let current_program_id_input = document.query_selector("#deploy-input-program-id").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let current_fee_input = document.query_selector("#deploy-input-fee").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                        let program_id = current_program_id_input.value().clone();
                        let fee = current_fee_input.value().clone();


                        let target1 = current_program_id_input.dyn_into::<HtmlElement>().unwrap();
                        let target2 = current_fee_input.dyn_into::<HtmlElement>().unwrap();


                        let style1 = target1.style();
                        let style2 = target2.style();

                        if &program_id== "" {
                            let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style1.set_property("border", "1px solid #494e64");   
                        }

                        if &fee == "" {
                            let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style2.set_property("border", "1px solid #494e64");   
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
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
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


#[component]
pub fn SidebarRestApi (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("get-latest-block-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Get Latest Block".to_string());

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#rest-api-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">REST API</div>
            <div id="rest-api-card" class="card" style="">
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


                                // let output_field = document.query_selector("#get-latest-block-output").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                // if output_field.inner_html() != "".to_string() {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().set_property("height", "100%");
                                // } else {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().remove_property("height");
                                // }


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

                                
                                // let output_field = document.query_selector("#get-block-by-height-output").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                // if output_field.inner_html() != "".to_string() {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().set_property("height", "100%");
                                // } else {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().remove_property("height");
                                // }

                        
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

                                let output_field = document.query_selector("#get-program-output").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                if output_field.inner_html() != "".to_string() {
                                    let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = card_element.style().set_property("height", "100%");
                                } else {
                                    let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = card_element.style().remove_property("height");
                                }

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

                                // let output_field = document.query_selector("#get-transaction-output").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                // if output_field.inner_html() != "".to_string() {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().set_property("height", "100%");
                                // } else {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().remove_property("height");
                                // }

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

                                // let output_field = document.query_selector("#get-account-balance-output").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                // if output_field.inner_html() != "".to_string() {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().set_property("height", "100%");
                                // } else {
                                //     let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                //     let _ = card_element.style().remove_property("height");
                                // }

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
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#get-block-by-height-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");   
                        }
                    }
                    >
                        Get
                    </button>
                </div>  
                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-program-button" {"display: flex"} else {"display: none"}}>
                    <div id="get-program-body" style="display:flex; flex-direction:column;" class="card-body">
                        <div class="input-field" style="order:1;">
                            <div class="field-title">Program ID</div>
                            <input id="get-program-input" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="get-program-input-error" class="error-title" style="display:none;"></div>
                        </div>
                        <div id="get-program-output-field" class="output-field" style="display:none; flex-direction:column; box-sizing:border-box; order:2; height:100%;">
                            <div style="order:0" class="field-title">Program</div>

                            <div class="output-textarea-wrapper">
                                <textarea style="order:0" id="get-program-output" placeholder="Program" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-textarea-img-wrapper" style="order:1">
                                    <CopyButton target_field="#get-program-output".to_string() element_type="TextArea".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="get-button" class="card-button"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#get-program-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");
                            let error = document.query_selector("#get-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error.style().set_property("display", "none");
                            
                            spawn_local(async move {
                                let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(),"--network".to_string(),"mainnet".to_string(),"--endpoint".to_string(),"https://api.explorer.provable.com/v1".to_string(),"program".to_string(), value]}).unwrap();
        
                                let output: (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                if output.0 {
                                    let mut formatted_output = String::new();
                                    let split = output.1.split("\n\n").collect::<Vec<&str>>();
                                    for item in &(split)[2..split.len()]{
                                        if *item == "" {
                                            formatted_output = format!("{}{}", formatted_output, "\n");
                                        } else {
                                            formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                        }
                                    }
    
                                    let output_element = document.query_selector("#get-program-output").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                    output_element.set_inner_html(&formatted_output); 
                                    let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let output_field = document.query_selector("#get-program-output-field").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = card_element.style().set_property("height", "100%");
                                    let _ = output_field.style().set_property("display","inline-block");    
                                } else {
                                    let error = document.query_selector("#get-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    error.set_inner_html("Error: The program with this ID does not exist.");
                                    let _ = error.style().set_property("display", "block");
                                }
                            });
                            
                        }
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
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#get-transaction-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");   
                        }
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
                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#get-account-balance-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = current_input.value().clone();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");   
                        }
                    }
                    >
                        Get
                    </button>
                </div>                           
            </div>
        </div>
    }
}


// #[component]
// pub fn SidebarDeploy (
//     selected_activity_icon: ReadSignal<String>
// ) -> impl IntoView {
//     view! {
//         <div class="wrapper" style={move || if selected_activity_icon.get() == "#deploy-button" {"display: flex;"} else {"display: none;"}}>
//             <div class="sidebar-title"> Deploy </div>
//         </div>
//     }
// }