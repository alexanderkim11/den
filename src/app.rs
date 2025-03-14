mod sidebar;
use sidebar::*;

mod ide;
use ide::*;

use leptos::web_sys::{Element,HtmlElement, HtmlTextAreaElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use syntect::{highlighting::Theme, parsing::SyntaxSet};
use wasm_bindgen::prelude::*;
use web_sys::css::escape;
use std::cmp;
use std::collections::HashMap;
use indexmap::IndexMap;

use std::hash::{DefaultHasher, Hash, Hasher};




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
struct LoadThemeArgs<'a> {
    code: &'a str,
}

#[derive(Serialize, Deserialize)]
struct WriteFileArgs<> {
    filepath : String,
    contents : String
}


#[derive(Serialize, Deserialize)]
pub struct PlaceholderArgs<> {
    pub placeholder : String,
}



/*
==============================================================================
AUXILLARY COMPONENTS
==============================================================================
*/


#[component]
fn FileTab(
    filepath: String,
    filename: String,
    selected_file: ReadSignal<String>,
    set_selected_file : WriteSignal<String>,
    open_files : ReadSignal<Vec<(String,String)>>,
    set_open_files : WriteSignal<Vec<(String,String)>>,
    saved_file_contents: ReadSignal<HashMap<String,String>>, 
    set_saved_file_contents: WriteSignal<HashMap<String,String>>,
    cached_file_contents: ReadSignal<HashMap<String,String>>, 
    set_cached_file_contents: WriteSignal<HashMap<String,String>>,
) -> impl IntoView {
    view! {
        <div class = "tab" id=filepath.clone()
        on:click:target = move|ev|{
            let target = ev.target().dyn_into::<Element>().unwrap();
            let new_val = Array::new();
            new_val.push(&serde_wasm_bindgen::to_value("selected").unwrap());
            let _ = target.class_list().add(&new_val);

            let document = leptos::prelude::document();
            let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

            let current_filepath = selected_file.get_untracked();
            let mut cached_content = cached_file_contents.get_untracked();

            cached_content.remove(&current_filepath);
            cached_content.insert(current_filepath, result_element.value());
            set_cached_file_contents.set(cached_content);

            if selected_file.get_untracked() != target.id(){
                set_selected_file.set(target.id());
            }
        }>
            {filename.clone()}
            <button class="exit-button"
            on:click:target = {
                let outer_filepath_clone = filepath.clone();
                let outer_filename_clone = filename.clone();
                move|ev|{
                    ev.stop_propagation();

                    let inner_filepath_clone = outer_filepath_clone.clone();
                    let inner_filename_clone = outer_filename_clone.clone();

                    let document = leptos::prelude::document();
                    let tab_id = format!("{}{}", "#", escape(&outer_filepath_clone));
                    let tab_element = document.query_selector(&tab_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                    let valid = tab_element.get_attribute("valid");

                    match valid {
                        //If valid == false
                        Some(_) => {
                            for index in 0..open_files.get_untracked().len(){
                                let mut vec = open_files.get_untracked();
                                if vec[index] == ((&inner_filepath_clone).to_string(),(&inner_filename_clone).to_string()){
                                    let mut saved_content = saved_file_contents.get_untracked();
                                    let mut cached_content = cached_file_contents.get_untracked();
                                    saved_content.remove(&inner_filepath_clone);
                                    cached_content.remove(&inner_filepath_clone);
        
                                    set_saved_file_contents.set(saved_content);
                                    set_cached_file_contents.set(cached_content);
        
                                    vec.remove(index);
                                    if vec.len() > 0 {
                                        if index == vec.len(){
                                            set_selected_file.set(vec[index-1].0.clone());
                                        } else if index < vec.len() {
                                            set_selected_file.set(vec[index].0.clone());
                                        }
                                    } else if vec.len() == 0 {
                                        set_selected_file.set(String::new());
                                    }
                                    set_open_files.set(vec);
                                    break;
                                }
                            }
                        }
                        None => {
                            spawn_local(async move {
                                let mut saved_content = saved_file_contents.get_untracked();
                                let cached_content = cached_file_contents.get_untracked();
                                let mut warning_result = String::new();
        
                                let selected = selected_file.get_untracked();
                                if selected == inner_filepath_clone {
                                    let document = leptos::prelude::document();
                                    let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                    let saved = saved_content.get(&inner_filepath_clone).unwrap().to_string();
                                    if saved != result_element.value() {
        
                                        let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();
                                        warning_result = invoke("warning", args).await.as_string().unwrap();
                
                                        if warning_result == "Save".to_string() {
                                            let document = leptos::prelude::document();
                                            let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();
                                            let args = serde_wasm_bindgen::to_value(&WriteFileArgs { filepath: inner_filepath_clone.clone(), contents: result_element.value()}).unwrap();
                                            let (_error, _message) : (bool, String) = serde_wasm_bindgen::from_value(invoke("write_file", args).await).unwrap();      
                                        }
                                    }
        
        
                                } else {
                                    let cached = cached_content.get(&inner_filepath_clone).unwrap().to_string();
                                    let saved = saved_content.get(&inner_filepath_clone).unwrap().to_string();
                                    if saved != cached {
        
                                        let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();
                                        warning_result = invoke("warning", args).await.as_string().unwrap();
                
                                        if warning_result == "Save".to_string() {
                                            let args = serde_wasm_bindgen::to_value(&WriteFileArgs { filepath: inner_filepath_clone.clone(), contents: cached.clone()}).unwrap();
                                            let (error, message) : (bool, String) = serde_wasm_bindgen::from_value(invoke("write_file", args).await).unwrap();
                                            if !error {
                                                saved_content.remove(&inner_filepath_clone);
                                                saved_content.insert(inner_filepath_clone.clone(), cached);
                                                set_saved_file_contents.set(saved_content);
                              
                                            } else {
                                                console_log(&message);
                                            }   
                                        }
                                    }
                                }
        
                                if warning_result != "Cancel".to_string(){
                                    for index in 0..open_files.get_untracked().len(){
                                        let mut vec = open_files.get_untracked();
                                        if vec[index] == ((&inner_filepath_clone).to_string(),(&inner_filename_clone).to_string()){
                                            let mut saved_content = saved_file_contents.get_untracked();
                                            let mut cached_content = cached_file_contents.get_untracked();
                                            saved_content.remove(&inner_filepath_clone);
                                            cached_content.remove(&inner_filepath_clone);
                
                                            set_saved_file_contents.set(saved_content);
                                            set_cached_file_contents.set(cached_content);
                
                                            vec.remove(index);
                                            if vec.len() > 0 {
                                                if index == vec.len(){
                                                    set_selected_file.set(vec[index-1].0.clone());
                                                } else if index < vec.len() {
                                                    set_selected_file.set(vec[index].0.clone());
                                                }
                                            } else if vec.len() == 0 {
                                                set_selected_file.set(String::new());
                                            }
                                            set_open_files.set(vec);
                                            break;
                                        }
                                    }
                                }
                            });

                        }
                    }
                }
            }
            >
                <div id={
                    let mut hasher = DefaultHasher::new();
                    format!("{}{}", filepath.clone(),"-unsaved").hash(&mut hasher);
                    format!("{}{}","p",hasher.finish().to_string())
                } style="display:none;" class="exit-img-wrapper">
                    <img class="unsaved-icon" src="public/circle-filled.svg"/> 
                    <img class="exit-icon" src="public/close.svg"/>
                </div>
                <div id={
                    let mut hasher = DefaultHasher::new();
                    format!("{}{}", filepath.clone(),"-saved").hash(&mut hasher);
                    format!("{}{}","p",hasher.finish().to_string())   
                } class="exit-img-wrapper">
                    <img style="display:flex;" class="exit-icon" src="public/close.svg"/>
                </div>

                {Effect::new({
                    let filepath_clone = filepath.clone();
                    move |_| {
                        let saved_content = saved_file_contents.get();
                        let cached_content = cached_file_contents.get();

                        let cached;
                        match cached_content.get(&filepath_clone){
                            Some(v) => cached = v.to_string(),
                            None => cached = String::new(),
                        }
                        let saved;
                        match saved_content.get(&filepath_clone){
                            Some(v) => saved = v.to_string(),
                            None => saved = String::new(),
                        }


                        let document = leptos::prelude::document();
                        let mut hasher = DefaultHasher::new();
                        format!("{}{}", filepath_clone,"-saved").hash(&mut hasher);      
                        let saved_id = format!("{}{}","#p",hasher.finish().to_string());
                        let mut hasher = DefaultHasher::new();
                        format!("{}{}", filepath_clone,"-unsaved").hash(&mut hasher);     
                        let unsaved_id = format!("{}{}","#p",hasher.finish().to_string());
                        if saved != cached {
                            let old_element = document.query_selector(&saved_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let new_element = document.query_selector(&unsaved_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();

                            let _ = old_element.style().set_property("display", "none");
                            let _ = new_element.style().remove_property("display");
                        } else {
                            let new_element = document.query_selector(&saved_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_element = document.query_selector(&unsaved_id).unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();

                            let _ = old_element.style().set_property("display", "none");
                            let _ = new_element.style().remove_property("display");



                        }
                    }
                });
                }
            </button>
        </div>

        {Effect::new(move |_| {
            let document = leptos::prelude::document();
            let target_string = format!("{}{}", "#", escape(&filepath));
            let target2 = document.query_selector(&target_string).unwrap();
            match target2 {
                Some(e) => {
                    let target = e;
                    let new_val = Array::new();
                    new_val.push(&serde_wasm_bindgen::to_value("selected").unwrap());
                    if selected_file.get() != target.id(){
                        let _ = target.class_list().remove(&new_val);
                    } else if selected_file.get() == target.id(){
                        let _ = target.class_list().add(&new_val);
                    }
                }
                None => {}

            }
        });
        }

    }
}


/*
==============================================================================
MAIN APP COMPONENT
==============================================================================
*/


#[component]
pub fn App() -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */


    // let (test, set_test) = signal(String::new());
    let (highlighted_msg, set_highlighted_msg) = signal(String::new());

    let (leo_syntax_set, set_leo_syntax_set) = signal(SyntaxSet::load_defaults_nonewlines());
    let (aleo_syntax_set, set_aleo_syntax_set) = signal(SyntaxSet::load_defaults_nonewlines());

    let (leo_theme, set_leo_theme) = signal(Theme::default());
    let (aleo_theme, set_aleo_theme) = signal(Theme::default());

    let (sl, set_sl) = signal(0i32);
    let (st, set_st) = signal(0i32);

    let (sidebar_offset_x, set_sidebar_offset_x) = signal(0i32);
    let (sidebar_dragging, set_sidebar_dragging) = signal(false);

    let (lines_html, set_lines_html) = signal("<button>1</button>".to_string());
    let (fs_html, set_fs_html) = signal(String::new());

    let (selected_activity_icon, set_selected_activity_icon) = signal("#environment-tab-button".to_string());
    let (selected_file, set_selected_file) = signal(String::new());
    let (open_files, set_open_files) : (ReadSignal<Vec<(String,String)>>,WriteSignal<Vec<(String,String)>>) = signal(Vec::new());

    // TODO: HashMap<String,(String, i32, i32, i32)> == filepath --> (contents, scroll_left, scroll_top, cursor_pos)
    let (saved_file_contents, set_saved_file_contents) : (ReadSignal<HashMap<String,String>>,WriteSignal<HashMap<String,String>>) = signal(HashMap::new());
    let (cached_file_contents, set_cached_file_contents) : (ReadSignal<HashMap<String,String>>,WriteSignal<HashMap<String,String>>) = signal(HashMap::new());

    let (environment_dropdown_active, set_environment_dropdown_active) = signal(false);
    let (current_environment_dropdown_item, set_current_environment_dropdown_item) = signal("devnet-button".to_string());
    let (current_environment_dropdown_text, set_current_environment_dropdown_text) = signal("Local Devnet".to_string());
    let (current_endpoint, set_current_endpoint) = signal("http://localhost:3030".to_string());

    // IndexMap from Name --> (private key, view key, address)
    let (accounts, set_accounts) : (ReadSignal<(IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>)>,WriteSignal<(IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>)>) = signal((IndexMap::new(),IndexMap::new()));

    let (compiled_project, set_compiled_project) = signal((String::new(),String::new()));
    let (root, set_root) = signal(String::new());

    /*
    ==============================================================================
    STARTUP EFFECTS
    ==============================================================================
    */


    // Load Syntax and Color Scheme from file
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&LoadThemeArgs { code : "null"}).unwrap();
        let return_tuple: (SyntaxSet, Theme) = serde_wasm_bindgen::from_value(invoke("load_leo_syntax", args).await).unwrap();
        set_leo_syntax_set.set(return_tuple.0);
        set_leo_theme.set(return_tuple.1);

        let args = serde_wasm_bindgen::to_value(&LoadThemeArgs { code : "null"}).unwrap();
        let return_tuple: (SyntaxSet, Theme) = serde_wasm_bindgen::from_value(invoke("load_aleo_syntax", args).await).unwrap();
        set_aleo_syntax_set.set(return_tuple.0);
        set_aleo_theme.set(return_tuple.1);

    });

    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();
        let (dev_accounts,saved_accounts) : (IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>) = serde_wasm_bindgen::from_value(invoke("get_state_accounts", args).await).unwrap();
        set_accounts.set((dev_accounts,saved_accounts));
    });
    
    spawn_local(async move {
        let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();
        invoke("start_dev_node", args).await;
    });
    
    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="main" 
        on:mousemove:target=move|ev|{
            if sidebar_dragging.get() {
                ev.prevent_default();
                let document = leptos::prelude::document();
                let x = ev.screen_x() - sidebar_offset_x.get();
                let result_element = document.query_selector(".sidebar-details").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                let style = result_element.style();


                let raw_val = style.get_property_value("flex-basis").unwrap();
                let val = raw_val[0..(raw_val.len()-2)].parse::<i32>().unwrap();

                let new_width = cmp::min(cmp::max(300, x + val),500);
                let new_val = format!("{}{}", new_width.to_string(), "px");
                let _ = style.set_property("flex-basis", &new_val);
                set_sidebar_offset_x.set(ev.screen_x());

            }

        }
        on:mouseup:target=move|ev|{
            if sidebar_dragging.get(){
                set_sidebar_dragging.set(false);
                let target = ev.target().dyn_into::<HtmlElement>().unwrap();
                let children = target.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    if child.class_name() != ".resizer" {
                        let _ = child.dyn_into::<HtmlElement>().unwrap().style().set_property("pointer-events","auto");
                    } 
                }
                let _ = target.style().set_property("cursor", "auto");
            }
        }
        on:mouseleave:target=move|ev|{
            if sidebar_dragging.get(){
                set_sidebar_dragging.set(false);
                let target = ev.target().dyn_into::<HtmlElement>().unwrap();
                let children = target.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    if child.class_name() != ".resizer" {
                        let _ = child.dyn_into::<HtmlElement>().unwrap().style().set_property("pointer-events","auto");
                    } 
                }
                let _ = target.style().set_property("cursor", "auto");
            }
        }
        >
            <div class="sidebar-icons">
                <div class ="temp-buffer"></div>
                <SidebarIcon selected=true id="environment-tab-button".to_string() style="padding:8px;".to_string() img_src="public/home.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon/>                
                <SidebarIcon id="file-explorer-tab-button".to_string() img_src="public/files.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="account-tab-button".to_string()  img_src="public/account.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="records-tab-button".to_string() style="padding:8px;".to_string() img_src="public/checklist.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="rest-api-tab-button".to_string() style="padding:8px;".to_string()  img_src="public/debug-disconnect.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon/>
                <SidebarIcon id="compile-tab-button".to_string()  img_src="public/extensions.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />                
                <SidebarIcon id="deploy-execute-tab-button".to_string() style="padding:8px;".to_string()  img_src="public/play-circle.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
                <SidebarIcon id="history-tab-button".to_string() style="padding:8px;".to_string()  img_src="public/history.svg".to_string() selected_activity_icon=selected_activity_icon set_selected_activity_icon=set_selected_activity_icon />
 
 
                <div id ="empty-space"></div>
                <button id ="settings-tab-button">
                    <img src="public/gear.svg"/>
                </button>

            </div>

            <div class="sidebar-details" style="display: flex; flex-basis: 300px;">
                <SidebarEnvironment selected_activity_icon=selected_activity_icon environment_dropdown_active=environment_dropdown_active set_environment_dropdown_active=set_environment_dropdown_active current_environment_dropdown_item=current_environment_dropdown_item set_current_environment_dropdown_item=set_current_environment_dropdown_item current_environment_dropdown_text=current_environment_dropdown_text set_current_environment_dropdown_text=set_current_environment_dropdown_text current_endpoint=current_endpoint set_current_endpoint=set_current_endpoint/>
                <SidebarFileExplorer selected_activity_icon=selected_activity_icon fs_html=fs_html set_fs_html=set_fs_html selected_file=selected_file set_selected_file=set_selected_file set_open_files=set_open_files cached_file_contents=cached_file_contents set_cached_file_contents=set_cached_file_contents root=root set_root=set_root set_highlighted_msg=set_highlighted_msg set_saved_file_contents=set_saved_file_contents/>
                <SidebarAccount selected_activity_icon=selected_activity_icon accounts=accounts set_accounts=set_accounts current_environment_dropdown_item=current_environment_dropdown_item/>
                <SidebarRecords selected_activity_icon=selected_activity_icon/>
                <SidebarRestApi selected_activity_icon=selected_activity_icon current_environment_dropdown_item=current_environment_dropdown_item current_endpoint=current_endpoint/>
                <SidebarCompile selected_activity_icon=selected_activity_icon selected_file=selected_file set_compiled_project=set_compiled_project current_environment_dropdown_item=current_environment_dropdown_item root=root set_fs_html=set_fs_html/>
                <SidebarDeployExecute selected_activity_icon=selected_activity_icon current_environment_dropdown_text=current_environment_dropdown_text current_endpoint=current_endpoint accounts=accounts  compiled_project=compiled_project  current_environment_dropdown_item=current_environment_dropdown_item/>
                <SidebarHistory selected_activity_icon=selected_activity_icon/>
            
            </div>



            <div class="resizer" 
            on:mousedown:target=move|ev|{
                ev.prevent_default();
                set_sidebar_dragging.set(true);
                set_sidebar_offset_x.set(ev.screen_x());

                let document = leptos::prelude::document();
                let result_element = document.query_selector(".main").unwrap().unwrap();
                let children = result_element.children();
                for i in 0..children.length(){
                    let child = children.get_with_index(i).unwrap();
                    if child.class_name() != ".resizer" {
                        let _ = child.dyn_into::<HtmlElement>().unwrap().style().set_property("pointer-events","none");
                    } 
                }
                let _ = result_element.dyn_into::<HtmlElement>().unwrap().style().set_property("cursor", "col-resize");
            }
            ></div>


            <div class= "code-terminal-area">
                <div class= "outer-code-area">
                    <div class= "tabs">
                        <For each=move || open_files.get() key=|tuple| tuple.0.clone() children=move |(filepath, filename)| {
                            view! {
                                <FileTab filepath=filepath filename=filename selected_file=selected_file set_selected_file=set_selected_file open_files=open_files set_open_files=set_open_files saved_file_contents=saved_file_contents set_saved_file_contents=set_saved_file_contents cached_file_contents=cached_file_contents set_cached_file_contents=set_cached_file_contents/>
                            }
                        }/>
                    </div>
                    <IDE lines_html=lines_html set_lines_html=set_lines_html sl=sl set_sl=set_sl st=st set_st=set_st highlighted_msg=highlighted_msg set_highlighted_msg=set_highlighted_msg leo_syntax_set=leo_syntax_set leo_theme=leo_theme aleo_syntax_set=aleo_syntax_set aleo_theme=aleo_theme selected_file=selected_file saved_file_contents=saved_file_contents set_saved_file_contents=set_saved_file_contents cached_file_contents=cached_file_contents set_cached_file_contents=set_cached_file_contents set_compiled_project=set_compiled_project current_environment_dropdown_item=current_environment_dropdown_item set_fs_html=set_fs_html root=root/>
                </div>
                <div class = "terminal">
                    //TODO: Start this
                </div>
            </div>
        </div>
    }
}
