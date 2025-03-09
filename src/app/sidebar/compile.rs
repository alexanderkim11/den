use leptos::web_sys::{Element,HtmlElement,HtmlButtonElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use std::path::Path;
use regex::Regex;

use crate::app::{generate_file_explorer_html, CustomDirEntry};

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
#[derive(Serialize, Deserialize)]
pub struct Command<> {
    pub command : Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct GetDirArgs<> {
    pub directory : String
}
/*
==============================================================================
COMPONENTS
==============================================================================
*/

#[component]
pub fn SidebarCompile (
    selected_activity_icon: ReadSignal<String>,
    selected_file : ReadSignal<String>,
    set_compiled_project : WriteSignal<(String,String)>,
    current_environment_dropdown_item : ReadSignal<String>,
    root : ReadSignal<String>,
    set_fs_html : WriteSignal<String>,
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */
    let (compiler_dropdown_active, set_compiler_dropdown_active) = signal(false);
    let (compiler_dropdown_item, set_compiler_dropdown_item) = signal("v241-button".to_string());
    let (compiler_dropdown_text, set_compiler_dropdown_text) = signal("2.4.1".to_string());

    let (compile_project_root, set_compile_project_root) = signal((String::new(),String::new()));

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#compile-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">Compile</div>
            
            <div class="sidebar-body-wrapper">
                <div id="compile-card" style="color:#e3e3e3;" class="card">
                    <div id="compile-card-head" class="card-head" >
                        <div class="title" style="-webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;">
                            Compile
                        </div>
                    </div>

                    <div class="card-body-wrapper">
                        <div id="compile-card-body" class="card-body">

                            <div class="input-field"  style="color:#e3e3e3;">
                                <div class="field-title">Compiler Version</div>
                                <div id="compiler-dropdown-custom" class="dropdown-custom">
                                    <div id="compiler-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                                    {
                                        let this = ev.target().dyn_into::<Element>().unwrap();
                                        let new_val = Array::new();
                                        new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                        if this.class_list().contains("show"){
                                            let _ = this.class_list().remove(&new_val);
                                            set_compiler_dropdown_active.set(false);
                                        } else {
                                            let _ = this.class_list().add(&new_val);
                                            set_compiler_dropdown_active.set(true);
                                        }
                                    }> 
                                        <div class="buffer" inner_html={move || compiler_dropdown_text.get()}></div>
                                        <img src="public/chevron-down.svg"/>
                                    </div>
                                    <div id="compiler-dropdown-content" class="dropdown-content" style={move || if compiler_dropdown_active.get() {"display: block"} else {"display: none"}}>
                                        <div id="v241-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if compiler_dropdown_item.get() == "v241-button" {"dropdown-item selected"} else {"dropdown-item"}}
                                        on:click:target = move|ev| {
                                            if compiler_dropdown_item.get() != ev.target().id(){
                                                set_compiler_dropdown_item.set(ev.target().id());
                                                set_compiler_dropdown_text.set(ev.target().inner_html());

                                                let document = leptos::prelude::document();
                                                let target = document.query_selector("#compiler-dropdown-button").unwrap().unwrap();
                                                let new_val = Array::new();
                                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                                let _ = target.class_list().remove(&new_val);
                                                set_compiler_dropdown_active.set(false);
                                            }
                                        }
                                        >
                                            2.4.1
                                        </div>
                                        // <For each=move || accounts.get() key=|(key,_)| key.to_string() children=move |(name,_)| {
                                        //     view! {
                                        //         <div id=name class={ let name_clone = name.clone(); move || { let id = saved_accounts_dropdown_item.get(); if id == name_clone  {"dropdown-item selected"} else {"dropdown-item"}}} style={ let name_clone = name.clone(); move || { let accounts_map = accounts.get(); if accounts_map.len() != 0 {let final_item = &accounts_map.get_index(accounts_map.len()-1).unwrap(); if final_item.0.to_string() == name_clone {"border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {""}} else {""}}}
                                        //         on:click:target = move|ev| {
                                        //             let current_item = saved_accounts_dropdown_item.get();
                                        //             if current_item != ev.target().id(){
                                        //                 set_saved_accounts_dropdown_item.set(ev.target().id());
                                        //                 set_saved_accounts_dropdown_text.set(ev.target().inner_html());
                        
                                        //                 let document = leptos::prelude::document();
                                        //                 let target = document.query_selector("#compiler-dropdown-button").unwrap().unwrap();
                                        //                 let new_val = Array::new();
                                        //                 new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                        //                 let _ = target.class_list().remove(&new_val);
                                        //                 set_saved_accounts_dropdown_active.set(false);
                                        //             }
                                        //         }

                                        //         >
                                        //             {name.clone()}
                                        //         </div>                                     
                                        //     }
                                        // }/>
                                    </div>
                                </div>
                            </div>


                            <div class="input-field">
                                <div class="field-title">Project</div>
                                <input id="project-root-input" value={move || compile_project_root.get().1} placeholder="--" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                            </div>
                        </div>

                        <div class="card-divider"/>

                        <button id="compile-button" class="card-button disabled"
                        on:click:target=move|ev| {
                            let document = leptos::prelude::document();
                            let target = document.query_selector("#compiler-output").unwrap().unwrap();
                            let _ = target.set_class_name("compile-output-message");
                            target.set_inner_html("");

                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let new_val2 = Array::new();
                            new_val2.push(&serde_wasm_bindgen::to_value("disabled").unwrap());
                            let _ = this.class_list().remove(&new_val2);

                            spawn_local(async move{
                                let network : String = if current_environment_dropdown_item.get_untracked() == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};
                                let args = serde_wasm_bindgen::to_value(&Command { command : vec!["build".to_string(), "--path".to_string(), compile_project_root.get_untracked().0, "--network".to_string(), network ,"--endpoint".to_string(),"https://api.explorer.provable.com/v1".to_string()]}).unwrap();
        
                                let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();

                                if !error {
                                    let args = serde_wasm_bindgen::to_value(&GetDirArgs { directory : root.get_untracked()}).unwrap();
                                    let return_val = invoke("get_directory", args).await.as_string().unwrap();
                                    let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
                                    let html_fs = generate_file_explorer_html(deserialized_return_val);
                                    set_fs_html.set(html_fs);

                                    let document = leptos::prelude::document();
                                    let target = document.query_selector("#compiler-output").unwrap().unwrap();
                                    let _ = target.set_class_name("compile-output-message success");
                                    let output = format!("{}{}{}", "\u{2713} Compiled '", compile_project_root.get_untracked().1, "' into Aleo instructions!");
                                    target.set_inner_html(&output);

                                    set_compiled_project.set(compile_project_root.get_untracked());

                                } else {
                                    set_compiled_project.set((String::new(),String::new()));

                                    let document = leptos::prelude::document();
                                    let target = document.query_selector("#compiler-output").unwrap().unwrap();
                                    let _ = target.set_class_name("compile-output-message failure");
                                    
                                    let split = output.split("\n").collect::<Vec<&str>>();
                                    let error_msg = split[0];

                                    let re = Regex::new(r"Error \[(?<error_code>[a-zA-Z0-9]*)\]:(?<explanation>.*)").unwrap();
                                    match re.captures(error_msg) {
                                        Some(caps) => {
                                            let response = format!("{}{}{}{}","Error ".to_string(), caps["error_code"].to_string(), ":",  caps["explanation"].to_string());
                                            target.set_inner_html(&response);
                                        },
                                        None => {
                                            target.set_inner_html("Unknown Error");
                                        }
                                    }

                                }
                                let _ = this.class_list().remove(&new_val);
                            });
                        }
                        >
                            Compile
                        </button> 

                        {Effect::new(move |_| {
                            let selected_filepath = selected_file.get().replace("\\", "/");
                            let path = Path::new(&selected_filepath);
                            let filename = path.file_name();

                            let document = leptos::prelude::document();
                            let target = document.query_selector("#compile-button").unwrap().unwrap().dyn_into::<HtmlButtonElement>().unwrap();
                            match filename {
                                Some(name) => {
                                    if name.to_str().unwrap() == "main.leo" {
                                        let src = path.parent().unwrap();
                                        let project_root = src.parent().unwrap();
                                        
                                        let _ = target.set_class_name("card-button");
                                        set_compile_project_root.set((project_root.to_str().unwrap().to_string(),project_root.file_name().unwrap().to_str().unwrap().to_string()));
                                    } else {
                                        let _ = target.set_class_name("card-button disabled");
                                        set_compile_project_root.set(("".to_string(),"".to_string()));

                                        let document = leptos::prelude::document();
                                        let target = document.query_selector("#compiler-output").unwrap().unwrap();
                                        let _ = target.set_class_name("compile-output-message");
                                        target.set_inner_html("")
                                    }
                                },
                                None => {
                                    let _ = target.set_class_name("card-button disabled");
                                    set_compile_project_root.set(("".to_string(),"".to_string()));

                                    let document = leptos::prelude::document();
                                    let target = document.query_selector("#compiler-output").unwrap().unwrap();
                                    let _ = target.set_class_name("compile-output-message");
                                    target.set_inner_html("")
                                }
                            }
                        });}

                    </div>
                </div>
                <div id="compiler-output" class="compile-output-message"></div>
            </div>
        </div>
    }
}