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
========
*/

/*
==============================================================================
COMPONENTS
==============================================================================
*/

#[component]
pub fn SidebarCompile (
    selected_activity_icon: ReadSignal<String>
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */


    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#compile-button" {"display: flex;"} else {"display: none;"}}>
        <div class="sidebar-title">Compile</div>


            <div id="compile-card" style="color:#e3e3e3;" class="card">
                <div id="compile-card-head" class="card-head" >
                    <div class="title" style="-webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;">
                        Compile
                    </div>
                </div>
                <div class="card-body-wrapper">
                    <div id="compile-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Project Root</div>
                            <input id="project-root-input" value="" placeholder="" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}