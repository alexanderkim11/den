use leptos::web_sys::{Element,HtmlElement,HtmlInputElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::app::CopyButton;

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
pub struct Command<> {
    pub command : Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct AddressFromViewKeyArgs<> {
    pub network : String,
    pub viewkey : String
}


/*
==============================================================================
COMPONENTS
==============================================================================
*/

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