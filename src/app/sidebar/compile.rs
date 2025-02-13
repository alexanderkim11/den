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
        </div>
    }
}