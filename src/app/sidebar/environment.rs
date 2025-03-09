use leptos::web_sys::Element;
use js_sys::Array;
use leptos::prelude::*;
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

/*
==============================================================================
COMPONENTS
==============================================================================
*/

#[component]
pub fn SidebarEnvironment (
    selected_activity_icon: ReadSignal<String>,

    environment_dropdown_active : ReadSignal<bool>,
    set_environment_dropdown_active : WriteSignal<bool>,

    current_environment_dropdown_item : ReadSignal<String>,
    set_current_environment_dropdown_item : WriteSignal<String>,

    current_environment_dropdown_text : ReadSignal<String>,
    set_current_environment_dropdown_text : WriteSignal<String>,

    current_endpoint : ReadSignal<String>,
    set_current_endpoint : WriteSignal<String>,

) -> impl IntoView {
    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#environment-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">Environment</div>


            <div class="sidebar-body-wrapper">
                <div id="environment-card" style="color:#e3e3e3;" class="card">
                    <div id="environment-card-head" class="card-head" >
                        <div class="title" style="-webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;">
                            Environment
                        </div>
                    </div>
                    <div class="card-body-wrapper">
                        <div id="environment-card-body" class="card-body">
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
                                                set_current_endpoint.set("http://localhost:3030".to_string());

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
                                                set_current_endpoint.set("https://api.explorer.provable.com/v1".to_string());

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
                                                set_current_endpoint.set("https://api.explorer.provable.com/v1".to_string());

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
                                <div class="field-title">Endpoint</div>
                                <input id="endpoint-input" value={move || current_endpoint.get()} placeholder="Endpoint" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
