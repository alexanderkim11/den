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
pub struct DecryptRecordArgs<> {
    pub network : String,
    pub ciphertext: String,
    pub viewkey : String
}


/*
==============================================================================
COMPONENTS
==============================================================================
*/


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
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#records-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">Records</div>
            
            <div class="sidebar-body-wrapper" style="height: 100%; overflow:visible;">
                <div id="records-card" style="height: 100%; box-sizing:border-box; margin:0px; padding:10px;" class="card">
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
                            // <div class="input-field" style="order:0;">
                            //     <div class="field-title">Name</div>
                            //     <input id="decrypt-record-input-name" placeholder="Record Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            // </div>
                            <div class="input-field" style="order:1;">
                                <div class="field-title">Ciphertext</div>
                                <input id="decrypt-record-input-ciphertext" placeholder="Ciphertext" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            </div>
                            <div class="input-field" style="order:2;">
                                <div class="field-title">View Key</div>
                                <input id="decrypt-record-input-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div id="decrypt-record-input-error" class="error-title" style="display:none;"></div>
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
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let document = leptos::prelude::document();
                            //let name_input_element = document.query_selector("#decrypt-record-input-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let ciphertext_input_element = document.query_selector("#decrypt-record-input-ciphertext").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_input_element = document.query_selector("#decrypt-record-input-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                            //let name = name_input_element.value();
                            let ciphertext = ciphertext_input_element.value();
                            let vk = vk_input_element.value();


                            //let target1 = name_input_element.dyn_into::<HtmlElement>().unwrap();
                            let target2 = ciphertext_input_element.dyn_into::<HtmlElement>().unwrap();
                            let target3 = vk_input_element.dyn_into::<HtmlElement>().unwrap();


                            //let style1 = target1.style();
                            let style2 = target2.style();
                            let style3 = target3.style();

                            // if &name == "" {
                            //     let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                            // } else {
                            //     let _ = style1.set_property("border", "1px solid #494e64");   
                            // }

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
                                let error = document.query_selector("#decrypt-record-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                let _ = style2.set_property("border", "1px solid #494e64");   
                                let _ = style3.set_property("border", "1px solid #494e64");  
                                spawn_local(async move{
                                    let args = serde_wasm_bindgen::to_value(&DecryptRecordArgs{network: "mainnet".to_string(), ciphertext : ciphertext, viewkey : vk}).unwrap();
                                    let (error,plaintext) : (bool, String) = serde_wasm_bindgen::from_value(invoke("decrypt_record", args).await).unwrap();
                                    if !error {
                                        let output_element = document.query_selector("#decrypt-record-output").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                        output_element.set_inner_html(&plaintext); 
                                    } else {
                                        let error = document.query_selector("#decrypt-record-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: Invalid View Key for this record.");
                                        let _ = error.style().set_property("display", "block");
                                    }
                                    let _ = this.class_list().remove(&new_val);
                                });
                            } else {
                                let _ = this.class_list().remove(&new_val);
                            }

                        }
                        >
                            Decrypt
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}