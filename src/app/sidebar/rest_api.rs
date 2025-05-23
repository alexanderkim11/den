use leptos::web_sys::{Element,HtmlElement,HtmlInputElement, HtmlTextAreaElement};
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
pub struct URL<> {
    pub url : String
}



/*
==============================================================================
COMPONENTS
==============================================================================
*/

#[component]
pub fn SidebarRestApi (
    selected_activity_icon: ReadSignal<String>,
    current_environment_dropdown_item : ReadSignal<String>,
    current_endpoint : ReadSignal<String>,

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
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#rest-api-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">REST API</div>

            
            <div class="sidebar-body-wrapper">
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


                                    let output_field = document.query_selector("#get-latest-block-output").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
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
                                Get Latest Block
                            </div>
                            <div id="get-block-by-height-button" class={move || if current_dropdown_item.get() == "get-block-by-height-button" {"dropdown-item selected"} else {"dropdown-item"}}
                            on:click:target = move|ev| {
                                if current_dropdown_item.get() != ev.target().id(){
                                    set_current_dropdown_item.set(ev.target().id());
                                    set_current_dropdown_text.set(ev.target().inner_html());

                                    let document = leptos::prelude::document();

                                    
                                    let output_field = document.query_selector("#get-block-by-height-output-json").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
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
                                Get Block By Height
                            </div>
                            <div id="get-program-button" class={move || if current_dropdown_item.get() == "get-program-button" {"dropdown-item selected"} else {"dropdown-item"}}
                            on:click:target = move|ev| {
                                if current_dropdown_item.get() != ev.target().id(){
                                    set_current_dropdown_item.set(ev.target().id());
                                    set_current_dropdown_text.set(ev.target().inner_html());
                                    
                                    let document = leptos::prelude::document();

                                    let output_field = document.query_selector("#get-program-output-json").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
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

                                    let output_field = document.query_selector("#get-transaction-output-json").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
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
                                Get Transaction
                            </div>
                            <div id="get-account-balance-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_dropdown_item.get() == "get-account-balance-button" {"dropdown-item selected"} else {"dropdown-item"}}
                            on:click:target = move|ev| {
                                if current_dropdown_item.get() != ev.target().id(){
                                    set_current_dropdown_item.set(ev.target().id());
                                    set_current_dropdown_text.set(ev.target().inner_html());

                                    let document = leptos::prelude::document();

                                    let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = card_element.style().remove_property("height");

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
                        <div id="get-latest-block-body" class="card-body" style="display:flex; flex-direction:column;">
                            <div id="get-latest-block-output-field" class="output-field" style="display:none; flex-direction:column; box-sizing:border-box; order:2; height:100%;">
                                <div style="order:0" class="field-title">JSON</div>

                                <div class="output-textarea-wrapper" style="box-sizing: border-box; padding-bottom:10px">
                                    <textarea style="order:0; white-space:normal;" id="get-latest-block-output" placeholder="None" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                    <div class="output-textarea-img-wrapper" style="order:1">
                                        <CopyButton target_field="#get-latest-block-output".to_string() element_type="TextArea".to_string()/>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <button id="get-latest-block-get-button" class="card-button"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);
        
                            let document = leptos::prelude::document();               
                            spawn_local(async move {
                                let network_item = current_environment_dropdown_item.get_untracked();
                                let network : String = if network_item == "mainnet-button" {"mainnet".to_string()} else  {"testnet".to_string()};
                                let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(), "block".to_string(), "--latest".to_string(), "--network".to_string(), network ,"--endpoint".to_string(),current_endpoint.get_untracked()]}).unwrap();
        
                                let (error, output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();

                                if !error {
                                    let mut formatted_output = String::new();
                                    let split = output.split("\n\n").collect::<Vec<&str>>();
                                    for item in &(split)[2..split.len()-2]{
                                        if *item == "" {
                                            formatted_output = format!("{}{}", formatted_output, "\n");
                                        } else {
                                            formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                        }
                                    }

                                    formatted_output = formatted_output.replace(",", ", ").replace("{", "{ ");

                                    let output_element = document.query_selector("#get-latest-block-output").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                    output_element.set_inner_html(&formatted_output); 
                                    let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let output_field = document.query_selector("#get-latest-block-output-field").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = card_element.style().set_property("height", "100%");
                                    let _ = output_field.style().set_property("display","inline-block");

                                    let old_button = document.query_selector("#get-latest-block-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_button = document.query_selector("#get-latest-block-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    
                                    let _ = old_button.style().set_property("display", "none");
                                    let _ = new_button.style().set_property("display", "flex");
                                    let _ = this.class_list().remove(&new_val);        
                                } else {
                                    let _ = this.class_list().remove(&new_val);    
                                }
                            });
                            
                        }
                        >
                            Get
                        </button>
                        <div id="get-latest-block-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                            <button id="get-latest-block-open-button" class="card-button" style="margin-right:10px;"
                            on:click:target=move|_ev| {
                                spawn_local(async move {
                                    let network_item = current_environment_dropdown_item.get_untracked();
                                    if network_item != "devnet-button" {
                                        let base_url : String = if network_item == "mainnet-button" {"https://explorer.provable.com/block/latest".to_string()} else {"https://testnet.explorer.provable.com/block/latest".to_string()};
                                        let args = serde_wasm_bindgen::to_value(&URL{url:base_url}).unwrap();
                                        invoke("open_url", args).await;                                   
                                    }

                                });
                            }
                            >
                                Open
                            </button>
                            <button id="get-latest-block-clear-button" class="card-button-clear" style="margin-left:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();

                                let old_button = document.query_selector("#get-latest-block-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let new_button = document.query_selector("#get-latest-block-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_button.style().set_property("display", "none");
                                let _ = new_button.style().set_property("display", "inline-block");    

                                let output_element = document.query_selector("#get-latest-block-output").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                output_element.set_inner_html(""); 

                                let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let output_field = document.query_selector("#get-latest-block-output-field").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = card_element.style().remove_property("height");
                                let _ = output_field.style().set_property("display", "none");
                            }
                            >
                                Clear
                            </button>
                        </div>
                    </div>
                    <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-block-by-height-button" {"display: flex"} else {"display: none"}}>
                        <div id="get-block-by-height-input-card-body" class="card-body" style="padding-bottom:0;">
                            <div class="input-field">
                                <div class="field-title">Block Height</div>
                                <input id="get-block-by-height-input" placeholder="Block Height" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div id="get-block-by-height-input-error" class="error-title" style="display:none;"></div>
                            </div>
                        </div>
                        <div id="get-block-by-height-output-card-body" class="card-body" style="display:none; flex-direction:column;">
                            <div id="get-block-by-height-output-field" class="output-field" style="display:flex; flex-direction:column; box-sizing:border-box; order:2; height:100%;">
                                <div class="output-field" style="display:flex; flex-direction: column;">
                                    <div class="field-title">Block Height</div>
                                    <div class="output-input-wrapper">
                                        <input id="get-block-by-height-output-height" style="border-radius:6px;" placeholder="Block Height" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                    </div>
                                </div>    
                                
                                <div style="order:0" class="field-title">JSON</div>

                                <div class="output-textarea-wrapper" style="box-sizing: border-box; padding-bottom:10px">
                                    <textarea style="order:0; white-space:normal;" id="get-block-by-height-output-json" placeholder="None" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                    <div class="output-textarea-img-wrapper" style="order:1">
                                        <CopyButton target_field="#get-block-by-height-output-json".to_string() element_type="TextArea".to_string()/>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="card-divider"/>
                        <button id="get-block-by-height-get-button" class="card-button"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#get-block-by-height-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = current_input.value();
                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            if &value == "" {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");
                                let _ = this.class_list().remove(&new_val);
                                let error = document.query_selector("#get-block-by-height-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");  
                            } else {
                                let _ = style.set_property("border", "1px solid #494e64");
                                let error = document.query_selector("#get-block-by-height-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                
                                spawn_local(async move {
                                    let network_item = current_environment_dropdown_item.get_untracked();
                                    let network : String = if network_item == "mainnet-button" {"mainnet".to_string()} else  {"testnet".to_string()};                                
                                    let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(), "block".to_string(), "--network".to_string(),network,"--endpoint".to_string(),current_endpoint.get_untracked(), value.clone()]}).unwrap();
            
                                    let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                    if !error {
                                        let mut formatted_output = String::new();
                                        let split = output.split("\n\n").collect::<Vec<&str>>();
                                        for item in &(split)[2..split.len()]{
                                            if *item == "" {
                                                formatted_output = format!("{}{}", formatted_output, "\n");
                                            } else {
                                                formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                            }
                                        }
        
                                        let json_output_element = document.query_selector("#get-block-by-height-output-json").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                        json_output_element.set_inner_html(&formatted_output);

                                        let height_output_element = document.query_selector("#get-block-by-height-output-height").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                        height_output_element.set_value(&value);

                                        let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let _ = card_element.style().set_property("height", "100%");

                                        let old_body = document.query_selector("#get-block-by-height-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_body = document.query_selector("#get-block-by-height-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_body.style().set_property("display", "none");
                                        let _ = new_body.style().set_property("display", "flex");

                                        let old_button = document.query_selector("#get-block-by-height-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_button = document.query_selector("#get-block-by-height-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_button.style().set_property("display", "none");
                                        let _ = new_button.style().set_property("display", "flex");        

                                        let _ = this.class_list().remove(&new_val);

                                    } else {
                                        let error = document.query_selector("#get-block-by-height-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: The block with this height does not exist.");
                                        let _ = error.style().set_property("display", "block");
                                        let _ = this.class_list().remove(&new_val);    
                                    }
                                });
                                
                            }
                        }
                        >
                            Get
                        </button>
                        <div id="get-block-by-height-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                            <button id="get-block-by-height-open-button" class="card-button" style="margin-right:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();
                                let current_input = document.query_selector("#get-block-by-height-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let value = current_input.value();
                                let network_item = current_environment_dropdown_item.get_untracked();
                                if network_item != "devnet-button" {
                                    let base_url : String = if network_item == "mainnet-button" {"https://explorer.provable.com/block/".to_string()} else {"https://testnet.explorer.provable.com/block/".to_string()};
                                    let url = format!("{}{}",base_url, value);
                                    spawn_local(async move {
                                        let args = serde_wasm_bindgen::to_value(&URL{url:url}).unwrap();
                                        invoke("open_url", args).await;
        
                                    });                               
                                }
                            }
                            >
                                Open
                            </button>
                            <button id="get-block-by-height-clear-button" class="card-button-clear" style="margin-left:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();

                                let old_button = document.query_selector("#get-block-by-height-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let new_button = document.query_selector("#get-block-by-height-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_button.style().set_property("display", "none");
                                let _ = new_button.style().set_property("display", "inline-block");    

                                let output_element = document.query_selector("#get-block-by-height-output-json").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                output_element.set_inner_html(""); 

                                let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = card_element.style().remove_property("height");

                                let new_body = document.query_selector("#get-block-by-height-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let old_body = document.query_selector("#get-block-by-height-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_body.style().set_property("display", "none");
                                let _ = new_body.style().set_property("display", "block");
                            }
                            >
                                Clear
                            </button>
                        </div>
                    </div>  


                    <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-program-button" {"display: flex"} else {"display: none"}}>
                        <div id="get-program-input-card-body" style="display:flex; flex-direction:column; padding-bottom:0;" class="card-body">
                            <div class="input-field" style="order:1;">
                                <div class="field-title">Program ID</div>
                                <input id="get-program-input" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div id="get-program-input-error" class="error-title" style="display:none;"></div>
                            </div>
                        </div>
                        <div id="get-program-output-card-body" class="card-body" style="display:none; flex-direction:column;">
                            <div id="get-program-output-field" class="output-field" style="display:flex; flex-direction:column; box-sizing:border-box; order:2; height:100%;">
                                <div class="output-field" style="display:flex; flex-direction: column;">
                                    <div class="field-title">Program ID</div>
                                    <div class="output-input-wrapper">
                                        <input id="get-program-output-id" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                        <div class="output-img-wrapper">
                                            <CopyButton target_field="#get-program-output-id".to_string() element_type="Input".to_string()/>
                                        </div>
                                    </div>
                                </div>    
                                
                                <div style="order:0" class="field-title">Program</div>

                                <div class="output-textarea-wrapper" style="box-sizing: border-box;">
                                    <textarea style="order:0" id="get-program-output-json" placeholder="Program" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                    <div class="output-textarea-img-wrapper" style="order:1">
                                        <CopyButton target_field="#get-program-output-json".to_string() element_type="TextArea".to_string()/>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="card-divider"/>

                        <button id="get-program-get-button" class="card-button"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#get-program-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = current_input.value().clone();
                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            if &value == "" {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");
                                let _ = this.class_list().remove(&new_val);
                                let error = document.query_selector("#get-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");   
                            } else {
                                let _ = style.set_property("border", "1px solid #494e64");
                                let error = document.query_selector("#get-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                
                                spawn_local(async move {
                                    let network_item = current_environment_dropdown_item.get_untracked();
                                    let network : String = if network_item == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};                                
                                    let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(),"--network".to_string(),network,"--endpoint".to_string(),current_endpoint.get_untracked(),"program".to_string(), value.clone()]}).unwrap();        
                                    let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                    if !error {
                                        let mut formatted_output = String::new();
                                        let split = output.split("\n\n").collect::<Vec<&str>>();
                                        for item in &(split)[2..split.len()-3]{
                                            if *item == "" {
                                                formatted_output = format!("{}{}", formatted_output, "\n");
                                            } else {
                                                formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                            }
                                        }
        
                                        let json_output_element = document.query_selector("#get-program-output-json").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                        json_output_element.set_inner_html(&formatted_output);

                                        let height_output_element = document.query_selector("#get-program-output-id").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                        height_output_element.set_value(&value);

                                        let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let _ = card_element.style().set_property("height", "100%");

                                        let old_body = document.query_selector("#get-program-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_body = document.query_selector("#get-program-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_body.style().set_property("display", "none");
                                        let _ = new_body.style().set_property("display", "flex");

                                        let old_button = document.query_selector("#get-program-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_button = document.query_selector("#get-program-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_button.style().set_property("display", "none");
                                        let _ = new_button.style().set_property("display", "flex");        

                                        let _ = this.class_list().remove(&new_val);
                                    } else {
                                        let error = document.query_selector("#get-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: The program with this ID does not exist.");
                                        let _ = error.style().set_property("display", "block");
                                        let _ = this.class_list().remove(&new_val);
                                    }
                                });
                                
                            }
                        }
                        >
                            Get
                        </button>
                        <div id="get-program-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                            <button id="get-program-open-button" class="card-button" style="margin-right:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();
                                let current_input = document.query_selector("#get-program-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let value = current_input.value();
                                let network_item = current_environment_dropdown_item.get_untracked();
                                if network_item != "devnet-button" {
                                    let base_url : String = if network_item == "mainnet-button" {"https://explorer.provable.com/program/".to_string()} else {"https://testnet.explorer.provable.com/program/".to_string()};
                                    let url = format!("{}{}",base_url, value);
                                    spawn_local(async move {
                                        let args = serde_wasm_bindgen::to_value(&URL{url:url}).unwrap();
                                        invoke("open_url", args).await;

                                    });
                                }
                            }
                            >
                                Open
                            </button>
                            <button id="get-program-clear-button" class="card-button-clear" style="margin-left:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();

                                let old_button = document.query_selector("#get-program-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let new_button = document.query_selector("#get-program-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_button.style().set_property("display", "none");
                                let _ = new_button.style().set_property("display", "inline-block");    

                                let output_element = document.query_selector("#get-program-output-json").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                output_element.set_inner_html(""); 

                                let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = card_element.style().remove_property("height");

                                let new_body = document.query_selector("#get-program-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let old_body = document.query_selector("#get-program-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_body.style().set_property("display", "none");
                                let _ = new_body.style().set_property("display", "block");
                            }
                            >
                                Clear
                            </button>
                        </div>
                    </div>

                    <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-transaction-button" {"display: flex"} else {"display: none"}}>
                        <div id="get-transaction-input-card-body" class="card-body" style="padding-bottom: 0;">
                            <div class="input-field">
                                <div class="field-title">Transaction ID</div>
                                <input id="get-transaction-input" placeholder="Transaction ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div id="get-transaction-input-error" class="error-title" style="display:none;"></div>
                            </div>
                        </div>
                        <div id="get-transaction-output-card-body" class="card-body" style="display:none; flex-direction:column;">
                            <div id="get-transaction-output-field" class="output-field" style="display:flex; flex-direction:column; box-sizing:border-box; order:2; height:100%;">
                                <div class="output-field" style="display:flex; flex-direction: column;">
                                    <div class="field-title">Transaction ID</div>
                                    <div class="output-input-wrapper">
                                        <input id="get-transaction-output-id" placeholder="Transaction ID" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                        <div class="output-img-wrapper">
                                            <CopyButton target_field="#get-transaction-output-id".to_string() element_type="Input".to_string()/>
                                        </div>
                                    </div>
                                </div>    
                                
                                <div style="order:0" class="field-title">JSON</div>

                                <div class="output-textarea-wrapper" style="box-sizing: border-box; padding-bottom:10px">
                                    <textarea style="order:0; white-space:normal;" id="get-transaction-output-json" placeholder="None" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                    <div class="output-textarea-img-wrapper" style="order:1">
                                        <CopyButton target_field="#get-transaction-output-json".to_string() element_type="TextArea".to_string()/>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <div class="card-divider"/>
                        <button id="get-transaction-get-button" class="card-button"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#get-transaction-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = current_input.value().clone();
                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            if &value == "" {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");
                                let _ = this.class_list().remove(&new_val);
                                let error = document.query_selector("#get-transaction-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                            } else {
                                
                                let _ = style.set_property("border", "1px solid #494e64");
                                let error = document.query_selector("#get-transaction-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                
                                spawn_local(async move {
                                    let network_item = current_environment_dropdown_item.get_untracked();
                                    let network : String = if network_item == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()} ;                                
                                    let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(), "transaction".to_string(), "--network".to_string(),network,"--endpoint".to_string(),current_endpoint.get_untracked(), value.clone()]}).unwrap();
            
                                    let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                    if !error {
                                        let mut formatted_output = String::new();
                                        let split = output.split("\n\n").collect::<Vec<&str>>();
                                        for item in &(split)[2..split.len()]{
                                            if *item == "" {
                                                formatted_output = format!("{}{}", formatted_output, "\n");
                                            } else {
                                                formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                            }
                                        }
        
                                        let json_output_element = document.query_selector("#get-transaction-output-json").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                        json_output_element.set_inner_html(&formatted_output);

                                        let height_output_element = document.query_selector("#get-transaction-output-id").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                        height_output_element.set_value(&value);

                                        let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let _ = card_element.style().set_property("height", "100%");

                                        let old_body = document.query_selector("#get-transaction-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_body = document.query_selector("#get-transaction-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_body.style().set_property("display", "none");
                                        let _ = new_body.style().set_property("display", "flex");
                                        
                                        let old_button = document.query_selector("#get-transaction-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_button = document.query_selector("#get-transaction-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_button.style().set_property("display", "none");
                                        let _ = new_button.style().set_property("display", "flex");        

                                        let _ = this.class_list().remove(&new_val);
                                    } else {
                                        let error = document.query_selector("#get-transaction-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: The transaction with this ID does not exist.");
                                        let _ = error.style().set_property("display", "block");
                                        let _ = this.class_list().remove(&new_val);
                                    }
                                });
                                
                            }
                        }
                        >
                            Get
                        </button>

                        <div id="get-transaction-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                            <button id="get-transaction-open-button" class="card-button" style="margin-right:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();
                                let current_input = document.query_selector("#get-transaction-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let value = current_input.value();
                                let network_item = current_environment_dropdown_item.get_untracked();
                                if network_item != "devnet-button" {
                                    let base_url : String = if network_item == "mainnet-button" {"https://explorer.provable.com/transaction/".to_string()} else {"https://testnet.explorer.provable.com/transaction/".to_string()};
                                    let url = format!("{}{}",base_url, value);
                                    spawn_local(async move {
                                        let args = serde_wasm_bindgen::to_value(&URL{url:url}).unwrap();
                                        invoke("open_url", args).await;

                                    });
                                }
                            }
                            >
                                Open
                            </button>
                            <button id="get-transaction-clear-button" class="card-button-clear" style="margin-left:10px;"
                            on:click:target=move|_ev| {
                                let document = leptos::prelude::document();

                                let old_button = document.query_selector("#get-transaction-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let new_button = document.query_selector("#get-transaction-get-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_button.style().set_property("display", "none");
                                let _ = new_button.style().set_property("display", "inline-block");    

                                let output_element = document.query_selector("#get-transaction-output-json").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                output_element.set_inner_html(""); 

                                let card_element = document.query_selector("#rest-api-card").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = card_element.style().remove_property("height");

                                let new_body = document.query_selector("#get-transaction-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let old_body = document.query_selector("#get-transaction-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_body.style().set_property("display", "none");
                                let _ = new_body.style().set_property("display", "block");
                            }
                            >
                                Clear
                            </button>
                        </div>
                    </div>  



                    <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "get-account-balance-button" {"display: flex"} else {"display: none"}}>
                        <div id="get-account-balance-body" class="card-body" style="padding-bottom: 0;">
                            <div class="input-field">
                                <div class="field-title">Address</div>
                                <input id="get-account-balance-input" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div id="get-account-balance-input-error" class="error-title" style="display:none;"></div>
                            </div>
                            <div class="output-field" id="get-account-balance-output-field" style="display:none;">
                                <div class="field-title">Balance</div>
                                <div class="output-input-wrapper">
                                    <input id="get-account-balance-output" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                    <div class="output-img-wrapper">
                                        <CopyButton target_field="#get-account-balance-output".to_string() element_type="Input".to_string()/>
                                    </div>
                                </div>
                            </div>
                        </div>
                        <div class="card-divider"/>
                        <button id="get-button" class="card-button"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#get-account-balance-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = current_input.value().clone();
                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            if &value == "" {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style.set_property("border", "1px solid #494e64");
                                let error = document.query_selector("#get-account-balance-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                let output_field = document.query_selector("#get-account-balance-output-field").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = output_field.style().set_property("display","none");

                                spawn_local(async move {
                                    let network_item = current_environment_dropdown_item.get_untracked();
                                    let network : String = if network_item == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()} ;                                
                                    let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(), "program".to_string(),"credits.aleo".to_string(),"--mapping-value".to_string(),"account".to_string(),value.clone() ,"--network".to_string(),network,"--endpoint".to_string(),current_endpoint.get_untracked(), "-q".to_string()]}).unwrap();
            
                                    let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                    if !error {
                                        let output_element = document.query_selector("#get-account-balance-output").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                        console_log(&output);
                                        let output_in_credits : f64 = output[..output.len()-7].parse::<u64>().expect("Failed to parse string to integer") as f64 / 1000000f64;
                                        let temp = output_in_credits.to_string();
                                        output_element.set_value(if output == "null" {"0"} else {&temp});
                                        let output_field = document.query_selector("#get-account-balance-output-field").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let _ = output_field.style().set_property("display","block");
                                        let _ = this.class_list().remove(&new_val);
                                    } else {
                                        let error = document.query_selector("#get-account-balance-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: Invalid Input");
                                        let _ = error.style().set_property("display", "block");
                                        let _ = this.class_list().remove(&new_val);
                                    }
                                });
                    
                            }
                        }
                        >
                            Get
                        </button>
                    </div>                           
                </div>
            </div>
        </div>
    }
}
