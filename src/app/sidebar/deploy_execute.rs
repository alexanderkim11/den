use leptos::web_sys::{Element,HtmlElement,HtmlInputElement};
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
    // accounts: ReadSignal<Vec<String>>

    environment_dropdown_active : ReadSignal<bool>,
    set_environment_dropdown_active : WriteSignal<bool>,

    current_environment_dropdown_item : ReadSignal<String>,
    set_current_environment_dropdown_item : WriteSignal<String>,

    current_environment_dropdown_text : ReadSignal<String>,
    set_current_environment_dropdown_text : WriteSignal<String>,
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("deploy-new-program-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Deploy a New Program".to_string());


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
                            <div id="environment-dropdown-custom" class="dropdown-custom" style="z-index: 3;">
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