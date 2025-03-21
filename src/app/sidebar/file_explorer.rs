use leptos::ev::Event;
use leptos::web_sys::{Element,HtmlElement, HtmlImageElement, HtmlTextAreaElement};
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlInputElement, KeyboardEvent, InputEvent};
use std::collections::HashMap;
use js_sys::Array;
use std::path::Path;
use regex::Regex;
use web_sys::css::escape;

use crate::app::{generate_file_explorer_html, CustomDirEntry};





#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct WriteFileArgs<> {
    filepath : String,
    contents : String
}

#[derive(Serialize, Deserialize)]
struct MkdirArgs<> {
    path: String,
}

/*
==============================================================================
STRUCTS
==============================================================================
*/


#[derive(Serialize, Deserialize)]
pub struct PlaceholderArgs<> {
    pub placeholder : String,
}

pub struct RecursiveClosureWrapper<'s>  {
    placeholder: Element,
    pub closure : &'s dyn Fn(Element,&RecursiveClosureWrapper)
}

#[derive(Serialize, Deserialize)]
pub struct GetDirArgs<> {
    pub directory : String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStateRootDirArgs<> {
    pub directory : String,
}

#[derive(Serialize, Deserialize)]
pub struct GetStateRootDirArgs<> {
    pub placeholder : String,
}


/*
==============================================================================
COMPONENTS
==============================================================================
*/



#[component]
pub fn SidebarFileExplorer (
    fs_html: ReadSignal<String>,
    set_fs_html : WriteSignal<String>,
    selected_activity_icon: ReadSignal<String>,
    selected_file : ReadSignal<String>,
    set_selected_file: WriteSignal<String>,
    set_open_files: WriteSignal<Vec<(String,String)>>,
    // saved_file_contents: ReadSignal<HashMap<String,String>>, 
    set_saved_file_contents: WriteSignal<HashMap<String,String>>,
    cached_file_contents: ReadSignal<HashMap<String,String>>, 
    set_cached_file_contents: WriteSignal<HashMap<String,String>>,
    root : ReadSignal<String>,
    set_root : WriteSignal<String>,
    set_highlighted_msg : WriteSignal<String>
) -> impl IntoView {

    let (fs_selected, set_fs_selected) = signal(String::new());


    /*
    ==============================================================================
    EVENT LISTENERS
    ==============================================================================
    */


    //Used for opening files
    let open_file_closure = Closure::wrap(Box::new(move |ev: Event| {
        let title_element = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        let dataset = title_element.dataset();
        let filepath = dataset.get("filepath").unwrap();
        let filename = Path::new(&filepath).file_name().unwrap().to_str().unwrap().to_string();


        let current_filepath = selected_file.get_untracked();
        let mut cached_content = cached_file_contents.get_untracked();

        let document = leptos::prelude::document();
        let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

        // Cache content of starting file before switching focus to new file
        cached_content.remove(&current_filepath);
        cached_content.insert(current_filepath,result_element.value());
        set_cached_file_contents.set(cached_content);

        set_selected_file.set(filepath.clone());
        set_open_files.update(|vec| if !vec.contains(&(filepath.clone(),filename.clone())){vec.push((filepath.clone(),filename.clone()))});
        
    }) as Box<dyn FnMut(_)>);


    //Used for opening files
    let open_file_closure_clone = Closure::wrap(Box::new(move |ev: Event| {
        let title_element = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        let dataset = title_element.dataset();
        let filepath = dataset.get("filepath").unwrap();
        let filename = Path::new(&filepath).file_name().unwrap().to_str().unwrap().to_string();


        let current_filepath = selected_file.get_untracked();
        let mut cached_content = cached_file_contents.get_untracked();

        let document = leptos::prelude::document();
        let result_element = document.query_selector(".editing").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

        // Cache content of starting file before switching focus to new file
        cached_content.remove(&current_filepath);
        cached_content.insert(current_filepath,result_element.value());
        set_cached_file_contents.set(cached_content);

        set_selected_file.set(filepath.clone());
        set_open_files.update(|vec| if !vec.contains(&(filepath.clone(),filename.clone())){vec.push((filepath.clone(),filename.clone()))});
        
    }) as Box<dyn FnMut(_)>);

    let fs_select_file_closure = Closure::wrap(Box::new(move |ev: Event| {
        let title_element = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        let dataset = title_element.dataset();
        let filepath = dataset.get("filepath").unwrap();
        set_fs_selected.set(filepath);


    }) as Box<dyn FnMut(_)>);

    let fs_select_file_closure_clone = Closure::wrap(Box::new(move |ev: Event| {
        let title_element = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
        let dataset = title_element.dataset();
        let filepath = dataset.get("filepath").unwrap();
        set_fs_selected.set(filepath);


    }) as Box<dyn FnMut(_)>);

    

    //Used for switching the chevron icon next to directories in the file system tab
    let switch_chevron_closure = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let children = this.children();
        let dir_element = this.parent_element().unwrap();
        set_fs_selected.set(this.id()[3..].to_string());

        let img = children.named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let dir_children = dir_element.children().named_item("children").unwrap().dyn_into::<HtmlElement>().unwrap();
        if img.class_name() == "inactive"{
            img.set_src("public/chevron-down.svg");
            img.set_class_name("active");
            let _ = dir_children.style().set_property("display", "flex");
        } else {
            img.set_src("public/chevron-right.svg");
            img.set_class_name("inactive");   
            let _ = dir_children.style().set_property("display", "none");
        }
    }) as Box<dyn FnMut(_)>);


    //Used for switching the chevron icon next to directories in the file system tab
    let switch_chevron_closure_clone = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let children = this.children();
        let dir_element = this.parent_element().unwrap();
        set_fs_selected.set(this.id()[3..].to_string());

        let img = children.named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
        let dir_children = dir_element.children().named_item("children").unwrap().dyn_into::<HtmlElement>().unwrap();
        if img.class_name() == "inactive"{
            img.set_src("public/chevron-down.svg");
            img.set_class_name("active");
            let _ = dir_children.style().set_property("display", "flex");
        } else {
            img.set_src("public/chevron-right.svg");
            img.set_class_name("inactive");   
            let _ = dir_children.style().set_property("display", "none");
        }
    }) as Box<dyn FnMut(_)>);


    let focus_file = Closure::wrap(Box::new(move |ev: Event| {
        let document = leptos::prelude::document();
        
        let target = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let filename = target.value();
        let dir = target.get_attribute("path").unwrap();
        let filepath = format!("{}{}{}",dir,"/",filename);

        let result_element = document.query_selector("#temp-file").unwrap().unwrap();
        result_element.remove();

        let valid = target.get_attribute("invalid");
        match valid {
            Some(_) => {}
            None => {
                if filename != String::new(){
                    let wrapper_element = document.create_element("div").expect("Error creating wrapper element");
                    wrapper_element.set_class_name("file");
        
                    let path = Path::new(&dir).join(filename.clone());
                    let path_str = path.to_str().unwrap();
        
                    let title_element = document.create_element("div").expect("Error creating title element");
                    let _ = title_element.set_attribute("name", "title");
                    let _ = title_element.set_id(&format!("{}{}","fs_",path_str));
                    let _ = title_element.set_attribute("data-filepath",path_str);
                    let _ = title_element.set_class_name("fs-title");
                    let _ = title_element.add_event_listener_with_callback("dblclick", open_file_closure_clone.as_ref().unchecked_ref());
                    let _ = title_element.add_event_listener_with_callback("click", fs_select_file_closure_clone.as_ref().unchecked_ref());
        
                    let text_element = document.create_element("div").expect("Error creating text element");
                    let text = document.create_text_node(&filename);
                    let _ = text_element.append_child(&text);
        
                    let img_element = document.create_element("img").expect("Error creating img element").dyn_into::<HtmlImageElement>().unwrap();
                    let extension = path.extension();
                    match extension {
                        Some(ext) => {
                            let ext_str = ext.to_str().unwrap();
                            if ext_str == "leo" {
                                let _ = img_element.set_src("public/leo.svg");
                                let _ = img_element.set_attribute("style","padding-left:2px; padding-top:1px;  padding-bottom:1px;  width:16px; height:15px;");
                                let _ = text_element.set_attribute("style", "padding-left:3.5px");
                
                            } else if ext_str == "aleo"{
                                let _ = img_element.set_src("public/aleo2.svg");
                                let _ = img_element.set_attribute("style","width:12px; height:13px; padding-top:2px; padding-left:5px; padding-bottom:2px; padding-right:2px;");
                            } else {
                                let _ = img_element.set_src("public/file.svg");
        
                            }
        
                        },
                        None => {
                            let _ = img_element.set_src("public/file.svg");
                        }
                    }
        
                    let _ = title_element.append_child(&img_element);
                    let _ = title_element.append_child(&text_element);
        
                    let _ = wrapper_element.append_child(&title_element);
        
                    let super_div = document.get_element_by_id(&format!("{}{}{}","fs_",&dir,"--children")).unwrap();
                    let children_list = super_div.children();
                    if children_list.length() > 0 {
                        let last_element = children_list.get_with_index(children_list.length()-1);
                        let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                    } else {
                        let _ = super_div.append_child(&wrapper_element);
                    }
        
        
        
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&WriteFileArgs {filepath: filepath, contents: String::new()}).unwrap();
                        let (error, message) : (bool, String) = serde_wasm_bindgen::from_value(invoke("write_file", args).await).unwrap();
        
        
                        //Open newly created file in IDE, set as selected
        
                        //Refresh File System Html
                    });
                }
            }
        }
    }) as Box<dyn FnMut(_)>);


    let focus_dir = Closure::wrap(Box::new(move |ev: Event| {
        let target = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let filename = target.value();
        let dir = target.get_attribute("path").unwrap();
        let filepath = format!("{}{}{}",dir,"/",filename);

        let document = leptos::prelude::document();
        let result_element = document.query_selector("#temp-dir").unwrap().unwrap();
        result_element.remove();

        let valid = target.get_attribute("invalid");
        match valid {
            Some(_) => {

            }
            None => {
                if filename != String::new(){
                    let wrapper_element = document.create_element("div").expect("Error creating wrapper element");
                    wrapper_element.set_class_name("dir");
        
        
                    let title_element = document.create_element("div").expect("Error creating title element");
                    let _ = title_element.set_attribute("name", "title");
                    let _ = title_element.set_id(&format!("{}{}","fs_",filepath));
                    let _ = title_element.set_attribute("data-filepath",&filepath);
                    let _ = title_element.set_class_name("fs-title");
                    let _ = title_element.add_event_listener_with_callback("click", switch_chevron_closure_clone.as_ref().unchecked_ref());
        
        
                    let img_element = document.create_element("img").expect("Error creating img element").dyn_into::<HtmlImageElement>().unwrap();
                    let _ = img_element.set_attribute("name", "image");
                    let _ = img_element.set_src("public/chevron-right.svg");
                    let _ = img_element.set_class_name("inactive");
                    let _ = img_element.set_id(&format!("{}{}", filepath, "--img"));
                    
                    let text_element = document.create_element("div").expect("Error creating text element");
                    let text = document.create_text_node(&filename);
                    let _ = text_element.append_child(&text);
        
                    let _ = title_element.append_child(&img_element);
                    let _ = title_element.append_child(&text_element);
        
        
                    let children_element = document.create_element("div").expect("Error creating children element");
                    let _ = children_element.set_attribute("name", "children");
                    let _ = children_element.set_id(&format!("{}{}{}","fs_",filepath, "--children"));
                    let _ = children_element.set_attribute("style","display:none");
                    let _ = children_element.set_class_name("dir-children");
                    let _ = title_element.add_event_listener_with_callback("click", switch_chevron_closure_clone.as_ref().unchecked_ref());
        
        
                    let _ = wrapper_element.append_child(&title_element);
                    let _ = wrapper_element.append_child(&children_element);
        
                    let super_div = document.get_element_by_id(&format!("{}{}{}","fs_",&dir,"--children")).unwrap();
                    let children_list = super_div.children();
                    if children_list.length() > 0 {
                        let mut inserted = false;
                        for index in 0..children_list.length() {
                            let element = children_list.get_with_index(index).unwrap();
                            if element.class_name() == "file"{
                                let _ = element.insert_adjacent_element("beforebegin", &wrapper_element);
                                inserted = true;
                                break;
                            }
                        }
                        if !inserted {
                            let last_element = children_list.get_with_index(children_list.length()-1);
                            let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                        }
                    } else {
                        let _ = super_div.append_child(&wrapper_element);
                    }
        
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&MkdirArgs {path: filepath}).unwrap();
                        let (error, message) : (bool, String) = serde_wasm_bindgen::from_value(invoke("mkdir", args).await).unwrap();
                        if error {
                            console_log(&message);
                        }
                    });
                }
            }
        }
    
    }) as Box<dyn FnMut(_)>);


    let input = Closure::wrap(Box::new(move |ev: InputEvent| {
        let target = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let filename = target.value();

        console_log(&filename);

        let dir = target.get_attribute("path").unwrap();
        let filepath = format!("{}{}{}",dir,"/",filename);

        let document = leptos::prelude::document();

        //TODO: Check if filepath already exists
        let formatted_id = format!("{}{}", "#fs_", escape(&filepath));
        let element_check = document.query_selector(&formatted_id);
        if filename != String::new(){
            match element_check {
                Ok(option) =>{
                    match option {
                        Some(_) => {
                            //Set errors
                            let casted = target.dyn_into::<HtmlElement>().expect("Error casting into HTML Element");

                            let _ = casted.style().set_property("border", "1px solid var(--grapefruit)");
                            let _ = casted.set_attribute("invalid", "true");
                            return
                        },
                        None => {
                            let casted = target.dyn_into::<HtmlElement>().expect("Error casting into HTML Element");

                            let _ = casted.style().remove_property("border");
                            let _ = casted.remove_attribute("invalid");
                        }
                    }
                }
                Err(_) => {
                    let casted = target.dyn_into::<HtmlElement>().expect("Error casting into HTML Element");

                    let _ = casted.style().remove_property("border");
                    let _ = casted.remove_attribute("invalid");                
                }
            }
        }

    }) as Box<dyn FnMut(_)>);

    let input_clone = Closure::wrap(Box::new(move |ev: InputEvent| {
        let target = ev.target().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        let filename = target.value();

        console_log(&filename);

        let dir = target.get_attribute("path").unwrap();
        let filepath = format!("{}{}{}",dir,"/",filename);

        let document = leptos::prelude::document();

        //TODO: Check if filepath already exists
        let formatted_id = format!("{}{}", "#fs_", escape(&filepath));
        let element_check = document.query_selector(&formatted_id);
        if filename != String::new(){
            match element_check {
                Ok(option) =>{
                    match option {
                        Some(_) => {
                            //Set errors
                            let casted = target.dyn_into::<HtmlElement>().expect("Error casting into HTML Element");

                            let _ = casted.style().set_property("border", "1px solid var(--grapefruit)");
                            let _ = casted.set_attribute("invalid", "true");
                            return
                        },
                        None => {
                            let casted = target.dyn_into::<HtmlElement>().expect("Error casting into HTML Element");

                            let _ = casted.style().remove_property("border");
                            let _ = casted.remove_attribute("invalid");
                        }
                    }
                }
                Err(_) => {
                    let casted = target.dyn_into::<HtmlElement>().expect("Error casting into HTML Element");

                    let _ = casted.style().remove_property("border");
                    let _ = casted.remove_attribute("invalid");                
                }
            }
        }
    }) as Box<dyn FnMut(_)>);


    let keydown = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
        let key = ev.key();
        if key == "Enter" {
            let target = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            let _ = target.blur();
        }
    }) as Box<dyn FnMut(_)>);

    let keydown_clone = Closure::wrap(Box::new(move |ev: KeyboardEvent| {
        let key = ev.key();
        if key == "Enter" {
            let target = ev.target().unwrap().dyn_into::<HtmlElement>().unwrap();
            let _ = target.blur();
        }
    }) as Box<dyn FnMut(_)>);

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#file-explorer-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">
                <div class="sidebar-title-text" style="white-space:nowrap;">File Explorer</div>
                <div id ="empty-space-horizontal"/>

                <button id="create-file-button" class="fs-button" style="display:none; margin-top:3px; margin-right:5px;"
                on:click:target=move|_ev| {
                    let document = leptos::prelude::document();

                    let mut selected = fs_selected.get_untracked();
                    let root = root.get_untracked().replace("\\","/");
                    if selected == String::new() {
                        selected = root.clone();
                    }

                    let wrapper_element = document.create_element("div").expect("Error creating wrapper element");
                    wrapper_element.set_class_name("file");
                    wrapper_element.set_id("temp-file");

                    let title_element2 = document.create_element("div").expect("Error creating title element");
                    let _ = title_element2.set_attribute("name", "title");
                    let _ = title_element2.set_class_name("temp-fs-title"); 

                    let img_element = document.create_element("img").expect("Error creating img element").dyn_into::<HtmlImageElement>().unwrap();
                    let _ = img_element.set_src("public/file.svg");

                    let new_content = document.create_element("input").expect("Error creating input element");
                    let _ = new_content.set_class_name("temp-fs-input");
                    let _ = new_content.set_attribute("spellcheck", "false");
                    let _ = new_content.set_attribute("autocomplete", "off");
                    let _ = new_content.set_attribute("path",&selected);

                    let _ = title_element2.append_child(&img_element);
                    let _ = title_element2.append_child(&new_content);

                    let _ = wrapper_element.append_child(&title_element2);

                    if selected == String::new() || selected == root {
                        let _ = new_content.set_attribute("path",&selected);

                        let formatted_id = format!("{}{}{}","fs_",root.replace("\\","/"),"--children");
                        let root_children = document.get_element_by_id(&formatted_id).unwrap();
                        let root_children2 = document.get_element_by_id(&formatted_id).unwrap();
                        let _  = root_children2.dyn_into::<HtmlElement>().unwrap().style().set_property("display","flex");
                        
                        let title_element = root_children.parent_element().unwrap().children().named_item("title").unwrap();
                        let img = title_element.children().named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
                        img.set_src("public/chevron-down.svg");
                        img.set_class_name("active");
                        

                        let children_list = root_children.children();
                        if children_list.length() > 0 {
                            let last_element = children_list.get_with_index(children_list.length()-1);
                            let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                        } else {
                            let _ = root_children.append_child(&wrapper_element);
                        }

                    } else {
                        let document = leptos::prelude::document();
                        let formatted_id = format!("{}{}","fs_",selected);
                        let parent;
                        match document.get_element_by_id(&formatted_id) {
                            Some(element) => {
                                match element.parent_element() {
                                    Some(element2) => {parent = element2}
                                    None => {
                                        let formatted_id = format!("{}{}","fs_",root.replace("\\","/"));
                                        let root = document.get_element_by_id(&formatted_id).expect("Root doesn't exist").parent_element().expect("Root parent doesn't exist");
                                        parent = root;
                                    }
                                }
                            }
                            None => {
                                let formatted_id = format!("{}{}","fs_",root.replace("\\","/"));
                                let root = document.get_element_by_id(&formatted_id).expect("Root doesn't exist").parent_element().expect("Root parent doesn't exist");
                                parent = root;
                            }
                        }
                        if parent.class_name() == "dir"{
                            let _ = new_content.set_attribute("path",&selected);

                            let children = parent.children().named_item("children").unwrap();
                            let _  = children.dyn_into::<HtmlElement>().unwrap().style().set_property("display","flex");
                            let title_element = parent.children().named_item("title").unwrap();
                            let img = title_element.children().named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
                            img.set_src("public/chevron-down.svg");
                            img.set_class_name("active");

                            let children2 = parent.children().named_item("children").unwrap();
                            let children_list = children2.children();


                            if children_list.length() > 0 {
                                let last_element = children_list.get_with_index(children_list.length()-1);
                                let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                            } else {
                                let _ = children2.append_child(&wrapper_element);
                            }

                        } else if parent.class_name() == "file" {
                            let super_children = parent.parent_element().unwrap();
                            let super_children_2 = parent.parent_element().unwrap();
                            let super_parent = super_children.parent_element().unwrap();

                            let id = super_children.id();

                            let re = Regex::new(r"fs_(?<path>.*)--children").unwrap();
                            let captures = re.captures(&id).expect("Error with regex");

                            let _ = new_content.set_attribute("path",&captures["path"]);

                            let title_element = super_parent.children().named_item("title").unwrap();
                            let img = title_element.children().named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
                            img.set_src("public/chevron-down.svg");
                            img.set_class_name("active");

                            let _ = super_children_2.dyn_into::<HtmlElement>().unwrap().style().set_property("display","flex");

                            let super_children_list = super_children.children();
                            if super_children_list.length() > 0 {
                                let last_element = super_children_list.get_with_index(super_children_list.length()-1);
                                let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                            } else {
                                let _ = super_children.append_child(&wrapper_element);
                            }
                        }
                    }

                    let new_content_html = new_content.dyn_into::<HtmlElement>().unwrap();
                    new_content_html.set_onblur(Some(focus_file.as_ref().unchecked_ref()));
                    let _ = new_content_html.add_event_listener_with_callback("input", input.as_ref().unchecked_ref());
                    let _ = new_content_html.add_event_listener_with_callback("keydown", keydown.as_ref().unchecked_ref());

                    let _ = new_content_html.focus();
                }
                >
                    <div id="create-file-img-wrapper" class="fs-img-wrapper">
                        <img id="create-file-icon" class="fs-icon" src="public/new-file.svg"/>
                    </div>
                </button>

                <button id="create-dir-button" class="fs-button" style="display:none; margin-top:2px; margin-right:5px;"
                on:click:target=move|_ev| {
                    let document = leptos::prelude::document();

                    let mut selected = fs_selected.get_untracked();
                    let root = root.get_untracked().replace("\\","/");
                    if selected == String::new() {
                        selected = root.clone();
                    }

                    let wrapper_element = document.create_element("div").expect("Error creating wrapper element");
                    wrapper_element.set_class_name("dir");
                    wrapper_element.set_id("temp-dir");

                    let title_element2 = document.create_element("div").expect("Error creating title element");
                    let _ = title_element2.set_attribute("name", "title");
                    let _ = title_element2.set_class_name("temp-fs-title"); 

                    let img_element = document.create_element("img").expect("Error creating img element").dyn_into::<HtmlImageElement>().unwrap();
                    let _ = img_element.set_src("public/chevron-right.svg");

                    let new_content = document.create_element("input").expect("Error creating input element");
                    let _ = new_content.set_class_name("temp-fs-input");
                    let _ = new_content.set_attribute("spellcheck", "false");
                    let _ = new_content.set_attribute("autocomplete", "off");
                    let _ = new_content.set_attribute("path",&selected);

                    let _ = title_element2.append_child(&img_element);
                    let _ = title_element2.append_child(&new_content);

                    let _ = wrapper_element.append_child(&title_element2);

                    if selected == String::new() || selected == root {
                        let _ = new_content.set_attribute("path",&selected);

                        let formatted_id = format!("{}{}{}","fs_",root.replace("\\","/"),"--children");
                        let root_children = document.get_element_by_id(&formatted_id).unwrap();
                        let root_children2 = document.get_element_by_id(&formatted_id).unwrap();
                        let _  = root_children2.dyn_into::<HtmlElement>().unwrap().style().set_property("display","flex");
                        
                        let title_element = root_children.parent_element().unwrap().children().named_item("title").unwrap();
                        let img = title_element.children().named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
                        img.set_src("public/chevron-down.svg");
                        img.set_class_name("active");
                        

                        let children_list = root_children.children();
                        if children_list.length() > 0 {
                            let last_element = children_list.get_with_index(children_list.length()-1);
                            let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                        } else {
                            let _ = root_children.append_child(&wrapper_element);
                        }

                    } else {
                        let document = leptos::prelude::document();
                        let formatted_id = format!("{}{}","fs_",selected);
                        let parent = document.get_element_by_id(&formatted_id).unwrap().parent_element().unwrap();
                        if parent.class_name() == "dir"{
                            let _ = new_content.set_attribute("path",&selected);

                            let children = parent.children().named_item("children").unwrap();
                            let _  = children.dyn_into::<HtmlElement>().unwrap().style().set_property("display","flex");
                            let title_element = parent.children().named_item("title").unwrap();
                            let img = title_element.children().named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
                            img.set_src("public/chevron-down.svg");
                            img.set_class_name("active");

                            let children2 = parent.children().named_item("children").unwrap();
                            let children_list = children2.children();


                            if children_list.length() > 0 {
                                let last_element = children_list.get_with_index(children_list.length()-1);
                                let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                            } else {
                                let _ = children2.append_child(&wrapper_element);
                            }

                        } else if parent.class_name() == "file" {
                            let super_children = parent.parent_element().unwrap();
                            let super_children_2 = parent.parent_element().unwrap();
                            let super_parent = super_children.parent_element().unwrap();

                            let id = super_children.id();

                            let re = Regex::new(r"fs_(?<path>.*)--children").unwrap();
                            let captures = re.captures(&id).expect("Error with regex");

                            let _ = new_content.set_attribute("path",&captures["path"]);

                            let title_element = super_parent.children().named_item("title").unwrap();
                            let img = title_element.children().named_item("image").unwrap().dyn_into::<HtmlImageElement>().unwrap();
                            img.set_src("public/chevron-down.svg");
                            img.set_class_name("active");

                            let _ = super_children_2.dyn_into::<HtmlElement>().unwrap().style().set_property("display","flex");

                            let super_children_list = super_children.children();
                            if super_children_list.length() > 0 {
                                let last_element = super_children_list.get_with_index(super_children_list.length()-1);
                                let _ = last_element.unwrap().insert_adjacent_element("afterend", &wrapper_element);
                            } else {
                                let _ = super_children.append_child(&wrapper_element);
                            }
                        }
                    }

                    let new_content_html = new_content.dyn_into::<HtmlElement>().unwrap();
                    new_content_html.set_onblur(Some(focus_dir.as_ref().unchecked_ref()));
                    let _ = new_content_html.add_event_listener_with_callback("input", input_clone.as_ref().unchecked_ref());
                    let _ = new_content_html.add_event_listener_with_callback("keydown", keydown_clone.as_ref().unchecked_ref());
                    let _ = new_content_html.focus();
                }



                >
                    <div id="create-dir-img-wrapper" class="fs-img-wrapper">
                        <img id="create-dir-icon" class="fs-icon" src="public/new-folder.svg"/>
                    </div>
                </button>

                <button id="change-dir-button" class="fs-button" style="display:none; margin-top:2px; margin-right:5px;"
                on:click:target=move|_ev| {

                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();
                        let warning_result = invoke("change_dir_warning", args).await.as_bool().unwrap();

                        if warning_result == true {

                            let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();
                            let return_val = invoke("open_explorer", args).await.as_string().unwrap();
                            if return_val != ""{

                                //Close all tabs and reset all fields
                                set_open_files.set(Vec::new());
                                set_highlighted_msg.set(String::new());
                                set_selected_file.set(String::new());
                                set_saved_file_contents.set(HashMap::new());
                                set_cached_file_contents.set(HashMap::new());



                                let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
    
                                set_root.set(deserialized_return_val[0].path.clone());
                                let args = serde_wasm_bindgen::to_value(&UpdateStateRootDirArgs { directory : deserialized_return_val[0].path.clone()}).unwrap();
                                invoke("update_state_root_dir", args).await;
    
                                set_fs_selected.set(deserialized_return_val[0].path.clone());
    
                                
                                let html_fs = generate_file_explorer_html(deserialized_return_val);
    
                                let document = leptos::prelude::document();
                                let element = document.query_selector(".open-folder-wrapper").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element.style().set_property("display", "none");
                                set_fs_html.set(html_fs);
    
                                let element2 = document.query_selector("#refresh-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element2.style().set_property("display", "flex");
    
                                let element3 = document.query_selector("#change-dir-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element3.style().set_property("display", "flex");
    
                                let element4 = document.query_selector("#create-file-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element4.style().set_property("display", "flex");
    
                                let element5 = document.query_selector("#create-dir-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element5.style().set_property("display", "flex");
                            }     
                        }
                    });
                }
                >
                    <div id="change-dir-img-wrapper" class="fs-img-wrapper">
                        <img id="change-dir-icon" class="fs-icon" src="public/folder-opened.svg"/>
                    </div>
                </button>
                <button id="refresh-button" class="fs-button" style="display:none"
                on:click:target = move |_ev| {
                    let root_dir = root.get_untracked();
                    if root_dir != "".to_string() {
                        spawn_local(async move{
                            set_fs_html.set(String::new());  //Stopgap solution until problem with updating fs html when new files/dirs are created is fixed           
                            let args = serde_wasm_bindgen::to_value(&GetDirArgs { directory : root.get_untracked()}).unwrap();
                            let return_val = invoke("get_directory", args).await.as_string().unwrap();
                            let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
                            let html_fs = generate_file_explorer_html(deserialized_return_val);
                            set_fs_html.set(html_fs);
                        });
                    }
                }
                >
                    <div id="refresh-img-wrapper" class="fs-img-wrapper">
                        <img id="refresh-icon" class="fs-icon" src="public/refresh.svg"/>
                    </div>
                </button>
            </div>
                
            <div class="sidebar-body-wrapper" style="overflow:auto;">
                <div class="open-folder-wrapper" style="display:flex;">
                    <button class="open-folder"
                    on:click:target=move|ev| {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                        let _ = this.class_list().add(&new_val);


                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&PlaceholderArgs { placeholder : String::new()}).unwrap();

                            let return_val = invoke("open_explorer", args).await.as_string().unwrap();
                            if return_val != ""{
                                let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");

                                set_root.set(deserialized_return_val[0].path.clone());
                                let args = serde_wasm_bindgen::to_value(&UpdateStateRootDirArgs { directory : deserialized_return_val[0].path.clone()}).unwrap();
                                invoke("update_state_root_dir", args).await;

                                
                                let fs_html = generate_file_explorer_html(deserialized_return_val);

                                let document = leptos::prelude::document();
                                let element = document.query_selector(".open-folder-wrapper").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element.style().set_property("display", "none");
                                set_fs_html.set(fs_html);

                                let element2 = document.query_selector("#refresh-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element2.style().set_property("display", "flex");

                                let element3 = document.query_selector("#change-dir-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element3.style().set_property("display", "flex");

                                let element4 = document.query_selector("#create-file-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element4.style().set_property("display", "flex");

                                let element5 = document.query_selector("#create-dir-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = element5.style().set_property("display", "flex");
                            }
                            let _ = this.class_list().remove(&new_val);
                        });
                    }
                    > Open Folder </button>
                </div>


                <div class="fs" inner_html={ move || fs_html.get() }></div>
                {Effect::new(move |_| {
                    let _signal = fs_html.get();
                    let document = leptos::prelude::document();
                    let result_element = document.query_selector(".fs").unwrap().unwrap();

                    //Recursively add event listeners to file explorer items
                    let wrapper = RecursiveClosureWrapper {
                        placeholder: result_element.clone(),
                        closure : &|element, closure|{
                            let children = element.children();
                            for i in 0..children.length(){
                                let child = children.get_with_index(i).unwrap();
                                if child.class_name() == "dir" {
                                    let title_element = child.children().named_item("title").unwrap();
                                    let _ = title_element.add_event_listener_with_callback("click", switch_chevron_closure.as_ref().unchecked_ref());
                                    (closure.closure)(child.children().named_item("children").unwrap(), closure);
                                } else if child.class_name() == "file" {
                                    let title_element = child.children().named_item("title").unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let _ = title_element.add_event_listener_with_callback("dblclick", open_file_closure.as_ref().unchecked_ref());
                                    let _ = title_element.add_event_listener_with_callback("click", fs_select_file_closure.as_ref().unchecked_ref());
                                }
                            }
                        }
                    };
                    (wrapper.closure)(result_element,&wrapper);
                });}

                {
                    spawn_local(async move {
                        let args = serde_wasm_bindgen::to_value(&GetStateRootDirArgs { placeholder: "null".to_string()}).unwrap();
                        let saved_root_dir =invoke("get_state_root_dir", args).await.as_string().unwrap();
                        if saved_root_dir != String::new() {
                            set_root.set(saved_root_dir.clone());
                            let args = serde_wasm_bindgen::to_value(&GetDirArgs { directory : saved_root_dir}).unwrap();
                            let return_val = invoke("get_directory", args).await.as_string().unwrap();
                            let deserialized_return_val : Vec<CustomDirEntry> = serde_json::from_str(&return_val).expect("Error with decoding dir_entry");
                            
                            
                            let html_fs = generate_file_explorer_html(deserialized_return_val);
                            let document = leptos::prelude::document();
                            let element = document.query_selector(".open-folder-wrapper").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = element.style().set_property("display", "none");
                            set_fs_html.set(html_fs);
        
                            let element2 = document.query_selector("#refresh-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = element2.style().set_property("display", "flex");

                            let element3 = document.query_selector("#change-dir-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = element3.style().set_property("display", "flex");

                            let element4 = document.query_selector("#create-file-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = element4.style().set_property("display", "flex");

                            let element5 = document.query_selector("#create-dir-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = element5.style().set_property("display", "flex");
                        }
                    });
                }
            </div>
        </div>
    }
}
