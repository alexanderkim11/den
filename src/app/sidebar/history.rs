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
pub fn SidebarHistory (
    selected_activity_icon: ReadSignal<String>,

) -> impl IntoView {
    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#history-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">History</div>

            <div class="sidebar-body-wrapper">
            </div>
        </div>
    }
}
