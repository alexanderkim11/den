use leptos::web_sys::{Element,HtmlElement,HtmlInputElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use indexmap::IndexMap;


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

/*
==============================================================================
COMPONENTS
==============================================================================
*/


#[component]
pub fn SidebarDeployExecute (
    selected_activity_icon: ReadSignal<String>,
    accounts : ReadSignal<IndexMap<String,(String,String,String)>>,
    set_accounts : WriteSignal<IndexMap<String,(String,String,String)>>,

) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("deploy-new-program-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Deploy a New Program".to_string());


    let (deploy_accounts_dropdown_active, set_deploy_accounts_dropdown_active) = signal(false);
    let (deploy_accounts_dropdown_item, set_deploy_accounts_dropdown_item) = signal(String::new());
    let (deploy_accounts_dropdown_text, set_deploy_accounts_dropdown_text) = signal("--".to_string());

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

                        <div class="input-field"  style="color:#e3e3e3;">
                            <div class="field-title">Account</div>
                            <div id="deploy-accounts-dropdown-custom" class="dropdown-custom">
                                <div id="deploy-accounts-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                                {
                                    let this = ev.target().dyn_into::<Element>().unwrap();
                                    let new_val = Array::new();
                                    new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                    if this.class_list().contains("show"){
                                        let _ = this.class_list().remove(&new_val);
                                        set_deploy_accounts_dropdown_active.set(false);
                                    } else {
                                        let _ = this.class_list().add(&new_val);
                                        set_deploy_accounts_dropdown_active.set(true);
                                    }
                                }> 
                                    <div class="buffer" inner_html={move || deploy_accounts_dropdown_text.get()}></div>
                                    <img src="public/chevron-down.svg"/>
                                </div>
                                <div id="deploy-accounts-dropdown-content" class="dropdown-content" style={move || if deploy_accounts_dropdown_active.get() {"display: block"} else {"display: none"}}>
                                    <div id="placeholder-button" class="dropdown-item-placeholder" style={move || if accounts.get().len() == 0 {"display: block; border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {"display: none; border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"}}
                                    >
                                        Please load an account first!
                                    </div>
                                    <For each=move || accounts.get() key=|(key,_)| key.to_string() children=move |(name,_)| {
                                        view! {
                                            <div id=name class={ let name_clone = name.clone(); move || { let id = deploy_accounts_dropdown_item.get(); if id == name_clone  {"dropdown-item selected"} else {"dropdown-item"}}} style={ let name_clone = name.clone(); move || { let accounts_map = accounts.get(); if accounts_map.len() != 0 {let final_item = &accounts_map.get_index(accounts_map.len()-1).unwrap(); if final_item.0.to_string() == name_clone {"border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {""}} else {""}}}
                                            on:click:target = move|ev| {
                                                let current_item = deploy_accounts_dropdown_item.get();
                                                if current_item != ev.target().id(){
                                                    set_deploy_accounts_dropdown_item.set(ev.target().id());
                                                    set_deploy_accounts_dropdown_text.set(ev.target().inner_html());
                    
                                                    let document = leptos::prelude::document();
                                                    let target = document.query_selector("#deploy-accounts-dropdown-button").unwrap().unwrap();
                                                    let new_val = Array::new();
                                                    new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                                    let _ = target.class_list().remove(&new_val);
                                                    set_deploy_accounts_dropdown_active.set(false);
                                                }
                                            }

                                            >
                                                {name.clone()}
                                            </div>                                     
                                        }
                                    }/>
                                </div>
                            </div>
                        </div>



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
                                <input type="checkbox"
                                on:change:target = move|ev|{
                                    let document = leptos::prelude::document();
                                                    
                                    let value = ev.target().checked();
                                    let private_fee_input_field = document.query_selector("#private-fee-input-field").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    if value {
                                        let _ = private_fee_input_field.style().set_property("display", "block");
                                    } else  {
                                        let _ = private_fee_input_field.style().set_property("display", "none");
                                    }         
                                }
                                />
                                <span class="slider round"></span>
                            </label>
                        </div>


                        <div class="input-field" id="private-fee-input-field" style="padding-top:10px; display: none;">
                            <div style="order:0" class="field-title">Fee Record</div>

                            <div class="output-textarea-wrapper" style="height:70px;">
                                <textarea style="order:0; white-space: pre-wrap; background-color:transparent; height: " id="private-fee-input" placeholder="Fee Record" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            </div>
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