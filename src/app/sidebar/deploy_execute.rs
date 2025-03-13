use leptos::web_sys::{Element,HtmlElement,HtmlInputElement, HtmlImageElement, HtmlTextAreaElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use indexmap::IndexMap;
use regex::Regex;
use leptos::ev::Event;

// use crate::app::CopyButton;


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
pub struct Command {
    pub command : Vec<String>
}

#[derive(Serialize, Deserialize)]
struct ReadProgramJsonArgs {
    filepath : String,
    field: String,
}

#[derive(Serialize, Deserialize)]
struct ReadFileArgs {
    filepath: String
}


#[derive(Serialize, Deserialize)]
struct RemoteExecuteArgs {
    program : String,
    function : String,
    network: String,
    endpoint : String,
    pk : String,
    inputs : Vec<String>,
}



/*
==============================================================================
COMPONENTS
==============================================================================
*/


#[component]
pub fn SidebarDeployExecute (
    selected_activity_icon: ReadSignal<String>,
    accounts : ReadSignal<(IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>)>,
    compiled_project : ReadSignal<(String,String)>,

    current_environment_dropdown_text : ReadSignal<String>,
    current_environment_dropdown_item : ReadSignal<String>,
    current_endpoint : ReadSignal<String>,
    
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

    let (compiled_program_id, set_compiled_program_id) = signal(String::new());

    let (new_loaded_program, set_new_loaded_program) = signal((String::new(), String::new(), String::new()));

    let (network_accounts, set_network_accounts) = signal(IndexMap::new());

    Effect::new({
        move || {
            if compiled_project.get().1 != String::new() {
                spawn_local(async move {
                    let args = serde_wasm_bindgen::to_value(&ReadProgramJsonArgs { filepath : format!("{}{}", compiled_project.get_untracked().0, "/program.json"), field: "program".to_string()}).unwrap();
                    let program_id = invoke("read_program_json", args).await.as_string().unwrap();
                    set_compiled_program_id.set(program_id.replace("\"",""));
                });                            
            }
        }
    });


    

    let program_compress_expand = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let card = this.parent_element().unwrap().parent_element().unwrap().parent_element().unwrap();  
        let img = this.dyn_into::<HtmlImageElement>().unwrap();
        let content = card.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
        if img.class_name() == "inactive"{ //Expand
            img.set_src("public/chevron-down.svg");
            img.set_class_name("active");
            card.set_class_name("program-card active");
            let _ = content.set_attribute("style","display: flex; border:0;");
        } else { //Compress
            img.set_src("public/chevron-right.svg");
            img.set_class_name("inactive");
            card.set_class_name("program-card");   
            let _ = content.set_attribute("style","display:none;");
        }
    }) as Box<dyn FnMut(_)>);
    
    let program_close = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let card = this.parent_element().unwrap().parent_element().unwrap().parent_element().unwrap();  
        card.remove();
    }) as Box<dyn FnMut(_)>);
    
    
    let function_compress = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let function_expanded= this.parent_element().unwrap().parent_element().unwrap();
        let function_compressed = function_expanded.parent_element().unwrap().children().get_with_index(0).unwrap();
    
        let _ = function_expanded.set_attribute("style", "display:none;");
        let _ = function_compressed.set_attribute("style", "");
    }) as Box<dyn FnMut(_)>);
    
    
    let function_expand = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let function_compressed = this.parent_element().unwrap().parent_element().unwrap();
        let function_expanded = function_compressed.parent_element().unwrap().children().get_with_index(1).unwrap();
    
        let _ = function_compressed.set_attribute("style", "display:none;");
        let _ = function_expanded.set_attribute("style", "");
    }) as Box<dyn FnMut(_)>);


    let function_call = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let function_wrapper = this.parent_element().unwrap().parent_element().unwrap().parent_element().unwrap();

        let function_name = function_wrapper.get_attribute("name").unwrap();
        let function_type = function_wrapper.get_attribute("function_type").unwrap();
        let program_name = function_wrapper.get_attribute("program_name").unwrap();

        if function_type == "async-function"  || function_type == "function" {
            let document = leptos::prelude::document();

            let current_fee_input = document.query_selector("#deploy-input-fee").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            let fee = current_fee_input.value().clone();
            let fee_record = document.query_selector("#private-fee-input").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value().clone();
            

            let target1 = current_fee_input.dyn_into::<HtmlElement>().unwrap();
            let target2 = document.query_selector("#deploy-accounts-dropdown-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
            let target3 = document.query_selector("#private-fee-input").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
            let private_fee_active = target3.get_attribute("active").unwrap();

            let style1 = target1.style();
            let style2 = target2.style();
            let style3 = target3.style();

            if &fee == "" {
                let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
            } else {
                let _ = style1.set_property("border", "1px solid #494e64");   
            }

            if deploy_accounts_dropdown_item.get_untracked() == String::new(){
                let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
            } else {
                let _ = style2.set_property("border", "1px solid #494e64");   
            }

            if private_fee_active == "true" && &fee_record == ""{
                let _ = style3.set_property("border", "1px solid var(--grapefruit)");   
            } else {
                let _ = style3.set_property("border", "1px solid #494e64");   
            }

 
            // TODO: CHECK IF THERE ARE ANY INPUTS

            // leo execute FUNCTION_NAME [INPUTS] --broadcast --endpoint ENDPOINT --network NETWORK --no-build --private-key PRIVATE_KEY [--fee FEE] [--record RECORD]
                

            if &fee != "" && deploy_accounts_dropdown_item.get_untracked() != String::new() && !(private_fee_active == "true" && &fee_record == ""){
                let _ = style1.set_property("border", "1px solid #494e64");  
                let _ = style2.set_property("border", "1px solid #494e64");  
                let _ = style3.set_property("border", "1px solid #494e64");  
                let mut inputs : Vec<(String,String)> = Vec::new();

                let children = function_wrapper.children();

                let new_val = Array::new();
                new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());

                let compressed_button = children.get_with_index(0).unwrap().children().get_with_index(0).unwrap().children().get_with_index(0).unwrap().dyn_into::<Element>().unwrap();
                let expanded_button = children.get_with_index(1).unwrap().children().get_with_index(2).unwrap().children().get_with_index(1).unwrap().dyn_into::<Element>().unwrap();

                let _ = compressed_button.class_list().add(&new_val);
                let _ = expanded_button.class_list().add(&new_val);  

                let function_output = children.get_with_index(2).unwrap().dyn_into::<Element>().unwrap();
                let function_error = children.get_with_index(3).unwrap().dyn_into::<Element>().unwrap();

                let _ = function_output.set_attribute("style","display:none");
                let _ = function_error.set_attribute("style","display:none");
                function_output.set_inner_html("");
                function_error.set_inner_html("");



                let function_compressed = children.get_with_index(0).unwrap().dyn_into::<HtmlElement>().unwrap();
                let compressed_style_display = function_compressed.style().get_property_value("display").expect("Error getting display property");
                if compressed_style_display == String::new(){
                    let function_compressed_input = function_compressed.children().get_with_index(0).unwrap().children().get_with_index(1).unwrap().dyn_into::<HtmlInputElement>().unwrap();
                    let val = function_compressed_input.value();
                    let types = function_compressed_input.placeholder();
                    let temp_inputs = val.split(",").collect::<Vec<&str>>();
                    let temp_types = types.split(",").collect::<Vec<&str>>();
                    for index in 0..temp_inputs.len(){
                        let input = temp_inputs[index].trim().to_string();
                        if index < temp_types.len(){
                            let input_type = temp_types[index].trim().to_string();
                            inputs.push((input,input_type));
                        } else {
                            inputs.push((input,String::new()));
                        }
                    }
                } else {
                    //Function compressed style is set to display:none, use function expanded inputs
                    let function_expanded = children.get_with_index(1).unwrap().dyn_into::<HtmlElement>().unwrap();
                    let expanded_style_display = function_expanded.style().get_property_value("display").expect("Error getting display property");
                    assert!(expanded_style_display == String::new());
    
                    let function_expanded_fields_wrapper = function_expanded.children().get_with_index(1).unwrap();
                    let function_expanded_fields_wrapper_children = function_expanded_fields_wrapper.children();
    
                    for index in 0..function_expanded_fields_wrapper_children.length(){
                        let function_expanded_field_wrapper = function_expanded_fields_wrapper_children.get_with_index(index).unwrap();
                        let input = function_expanded_field_wrapper.children().get_with_index(1).unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let val = input.value();
                        let input_type = input.placeholder();
                        inputs.push((val,input_type));
    
                    }
                }


                spawn_local(async move {
                    let network : String = if current_environment_dropdown_item.get_untracked() == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};
                    
                    let accounts_map = network_accounts.get_untracked();
                    let account : &(String, String, String) = accounts_map.get(&deploy_accounts_dropdown_item.get_untracked()).unwrap();
                    let pk = account.0.clone();
                    let current_fee_input = document.query_selector("#deploy-input-fee").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                    let fee = current_fee_input.value().clone();
                    let fee_in_microcredits : u64; //* 1000000;
                    match fee.parse::<u64>() {
                        Ok(val) => {fee_in_microcredits = val *1000000},
                        Err(_) => {fee_in_microcredits = 0},
                    }
                    let mut formatted_inputs : Vec<String> = Vec::new();
    
                    let input_type_re = Regex::new(r"(?<raw_type>.*)\.(public|private)").unwrap();
                    let algebra_primitives = ["i8", "i16", "i32", "i64", "i128","u8", "u16", "u32", "u64", "u128","scalar","field","group"];
                    for input in inputs {
                        let raw_input : String;
                        let input_value = input.0;
                        let input_type = input.1;
                        match input_type_re.captures(&input_type){
                            Some(caps) => {
                                let raw_type = &caps["raw_type"];
                                if algebra_primitives.contains(&raw_type){
                                    let input_value_re = Regex::new(&format!("{}{}","[0-9]+",raw_type)).unwrap();
                                    match input_value_re.captures(&input_value){
                                        Some(_) => {
                                            raw_input = input_value;
                                        },
                                        None => {
                                            raw_input = format!("{}{}",input_value,raw_type);
                                        }
                                    }
                                } else {
                                    raw_input = input_value;
                                }
                            },
                            None => {
                                if input_type != String::new() {
                                    raw_input = input_value;
                                } else {
                                    panic!("Error: no type matches.  This is a bug.")
                                }
                            }
                        }
                        formatted_inputs.push(raw_input);
                    }
    
                    let test = serde_wasm_bindgen::to_value(
                        &RemoteExecuteArgs {
                            program : program_name.clone(),
                            function : function_name.clone(),
                            network: network.clone(),
                            endpoint : current_endpoint.get_untracked(),
                            pk : pk.clone(),
                            inputs : formatted_inputs.clone()
                        }
                    ).unwrap();

                    let (error,outputs): (bool, Vec<String>) = serde_wasm_bindgen::from_value(invoke("execute_remote_wrapper", test).await).unwrap();

                    let mut command = vec!["execute".to_string(), function_name];
                    command.append(&mut formatted_inputs);
                    command.append(&mut vec!["--program".to_string(),program_name,"--broadcast".to_string(),"--yes".to_string(),"--private-key".to_string(), pk,"--base-fee".to_string(),fee_in_microcredits.to_string(),"--network".to_string(),network.clone(),"--endpoint".to_string(),current_endpoint.get_untracked()]);
                    if private_fee_active == "true" {
                        command.push("--record".to_string());
                        command.push(fee_record);
                    }
                    
                    let args = serde_wasm_bindgen::to_value(&Command { command : command}).unwrap();        
                    let (_,_): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();

                    if !error {
                        let function_output = children.get_with_index(2).unwrap().dyn_into::<Element>().unwrap();
                        if outputs.len() == 0{
                            function_output.set_inner_html("No outputs");
                        } else if outputs.len() == 1 {
                            function_output.set_inner_html(&format!("{}{}", "Output: ", outputs[0]));
                        } else {
                            function_output.set_inner_html(&format!("{}{}", "Outputs: ", outputs.join(", ")));
                        }
                        let _ = function_output.set_attribute("style","");
                    } else {
                        let function_error = children.get_with_index(3).unwrap().dyn_into::<Element>().unwrap();
                        function_error.set_inner_html("Error: Function call failed");
                        let _ = function_error.set_attribute("style","");
                    }

                    let _ = compressed_button.class_list().remove(&new_val);
                    let _ = expanded_button.class_list().remove(&new_val);  
                });

            }


        }  else if function_type == "mapping" {
            //let document = leptos::prelude::document();

            // let current_fee_input = document.query_selector("#deploy-input-fee").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            // let fee = current_fee_input.value().clone();
            // let fee_record = document.query_selector("#private-fee-input").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap().value().clone();
            

            // let target1 = current_fee_input.dyn_into::<HtmlElement>().unwrap();
            // let target2 = document.query_selector("#deploy-accounts-dropdown-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
            // let target3 = document.query_selector("#private-fee-input").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
            // let private_fee_active = target3.get_attribute("active").unwrap();

            // let style1 = target1.style();
            // let style2 = target2.style();
            // let style3 = target3.style();

            // if &fee == "" {
            //     let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
            // } else {
            //     let _ = style1.set_property("border", "1px solid #494e64");   
            // }

            // if deploy_accounts_dropdown_item.get_untracked() == String::new(){
            //     let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
            // } else {
            //     let _ = style2.set_property("border", "1px solid #494e64");   
            // }

            // if private_fee_active == "true" && &fee_record == ""{
            //     let _ = style3.set_property("border", "1px solid var(--grapefruit)");   
            // } else {
            //     let _ = style3.set_property("border", "1px solid #494e64");   
            // }
 
            // TODO: CHECK IF THERE ARE ANY INPUTS

            if true { //&fee != "" && deploy_accounts_dropdown_item.get_untracked() != String::new() && !(private_fee_active == "true" && &fee_record == ""){                
                // let _ = style1.set_property("border", "1px solid #494e64");  
                // let _ = style2.set_property("border", "1px solid #494e64");  
                // let _ = style3.set_property("border", "1px solid #494e64");  

                let children = function_wrapper.children();

                let new_val = Array::new();
                new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());

                let compressed_button = children.get_with_index(0).unwrap().children().get_with_index(0).unwrap().children().get_with_index(0).unwrap().dyn_into::<Element>().unwrap();
                let expanded_button = children.get_with_index(1).unwrap().children().get_with_index(2).unwrap().children().get_with_index(1).unwrap().dyn_into::<Element>().unwrap();

                let _ = compressed_button.class_list().add(&new_val);
                let _ = expanded_button.class_list().add(&new_val);  

                let function_output = children.get_with_index(2).unwrap().dyn_into::<Element>().unwrap();
                let function_error = children.get_with_index(3).unwrap().dyn_into::<Element>().unwrap();

                let _ = function_output.set_attribute("style","display:none");
                let _ = function_error.set_attribute("style","display:none");
                function_output.set_inner_html("");
                function_error.set_inner_html("");


                let input : String;
                let input_type : String;

                let children = function_wrapper.children();
                let function_output = children.get_with_index(2).unwrap().dyn_into::<Element>().unwrap();
                let function_error = children.get_with_index(3).unwrap().dyn_into::<Element>().unwrap();
                function_output.set_inner_html("");
                let _ = function_output.set_attribute("style","display:none");
                function_error.set_inner_html("");
                let _ = function_error.set_attribute("style","display:none");


                let function_compressed = children.get_with_index(0).unwrap().dyn_into::<HtmlElement>().unwrap();
                let compressed_style_display = function_compressed.style().get_property_value("display").expect("Error getting display property");
                if compressed_style_display == String::new(){
                    let function_compressed_input = function_compressed.children().get_with_index(0).unwrap().children().get_with_index(1).unwrap().dyn_into::<HtmlInputElement>().unwrap();
                    input = function_compressed_input.value();
                    input_type = function_compressed_input.placeholder();
                } else {
                    //Function compressed style is set to display:none, use function expanded inputs
                    let function_expanded = children.get_with_index(1).unwrap().dyn_into::<HtmlElement>().unwrap();
                    let expanded_style_display = function_expanded.style().get_property_value("display").expect("Error getting display property");
                    assert!(expanded_style_display == String::new());

                    let function_expanded_input = function_expanded.children().get_with_index(1).unwrap().children().get_with_index(0).unwrap().children().get_with_index(1).unwrap().dyn_into::<HtmlInputElement>().unwrap();
                    input = function_expanded_input.value();
                    input_type = function_expanded_input.placeholder();
                }

                let input_type_re = Regex::new(r"(?<raw_type>.*)\.(public|private)").unwrap();
                let raw_input : String;
                match input_type_re.captures(&input_type){
                    Some(caps) => {
                        let raw_type = &caps["raw_type"];
                        let algebra_primitives = ["i8", "i16", "i32", "i64", "i128","u8", "u16", "u32", "u64", "u128","scalar","field","group"];
                        if algebra_primitives.contains(&raw_type){
                            let input_value_re = Regex::new(&format!("{}{}","[0-9]+",raw_type)).unwrap();
                            match input_value_re.captures(&input){
                                Some(_) => {
                                    raw_input = input;
                                },
                                None => {
                                    raw_input = format!("{}{}",input,raw_type);
                                }
                            }
                        } else {
                            raw_input = input;
                        }
                    },
                    None => {
                        panic!("Error: no type matches.  This is a bug.")
                    }
                }


                spawn_local(async move {
                    let network : String = if current_environment_dropdown_item.get_untracked() == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};
                    let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(),"program".to_string(),program_name,"--mapping-value".to_string(),function_name, raw_input, "--network".to_string(),network.clone(),"--endpoint".to_string(),current_endpoint.get_untracked(), "-q".to_string()]}).unwrap();        
                    let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                    if !error {
                        let function_output = children.get_with_index(2).unwrap().dyn_into::<Element>().unwrap();
                        function_output.set_inner_html(&format!("{}{}", "Output: ", output));
                        let _ = function_output.set_attribute("style","");
                    } else {
                        let function_error = children.get_with_index(3).unwrap().dyn_into::<Element>().unwrap();
                        function_error.set_inner_html("Error: Value not found");
                        let _ = function_error.set_attribute("style","");
                    }

                    let _ = compressed_button.class_list().remove(&new_val);
                    let _ = expanded_button.class_list().remove(&new_val);  
                });
            }   
        }
    }) as Box<dyn FnMut(_)>);

    
    Effect::new({
        move || {
            let program = new_loaded_program.get();
            if program.2 != String::new() {
                let document = leptos::prelude::document();
                /*
                ===========================
                    PROGRAM START 
                ===========================
                */
                let program_card = document.create_element("div").expect("Error creating program card");
                program_card.set_class_name("program-card");
                let program_title = format!("{}{}{}{}",program.0," (", program.1, ")");
                program_card.set_id(&program_title);
                
                /*
                ===========================
                    HEADER CODE HERE
                ===========================
                */
                
                let program_custom_head = document.create_element("div").expect("Error creating program custom head");
                program_custom_head.set_class_name("program-custom-head");
                
                let program_dropdown_button = document.create_element("div").expect("Error creating program dropdown button");
                program_dropdown_button.set_class_name("dropdown-button");
                
                let header_img_expand = document.create_element("img").expect("Error creating header img expand element").dyn_into::<HtmlImageElement>().unwrap();
                let _ = header_img_expand.set_src("public/chevron-right.svg");
                header_img_expand.set_class_name("inactive");
                let _ = header_img_expand.set_attribute("style","order:1; z-index: 2;");
                let _ = header_img_expand.add_event_listener_with_callback("click", program_compress_expand.as_ref().unchecked_ref());
                
                
                let buffer = document.create_element("div").expect("Error creating program header buffer element");
                buffer.set_class_name("buffer");
                let _ = buffer.set_attribute("style", "order:2");
                let text = document.create_text_node(&program_title);
                let _ = buffer.append_child(&text);
                
                let header_img_close = document.create_element("img").expect("Error creating header img close element").dyn_into::<HtmlImageElement>().unwrap();
                let _ = header_img_close.set_src("public/close.svg");
                header_img_close.set_class_name("inactive");
                let _ = header_img_close.set_attribute("style","order:3; z-index: 2;");
                let _ = header_img_close.add_event_listener_with_callback("click", program_close.as_ref().unchecked_ref());
                
                let _ = program_dropdown_button.append_child(&header_img_expand);
                let _ = program_dropdown_button.append_child(&buffer);
                let _ = program_dropdown_button.append_child(&header_img_close);
                let _ = program_custom_head.append_child(&program_dropdown_button);
                
                /*
                ===========================
                END HEADER CODE 
                ===========================
                */
                
                let card_body_wrapper = document.create_element("div").expect("Error creating card body wrapper");
                card_body_wrapper.set_class_name("card-body-wrapper");
                let _ = card_body_wrapper.set_attribute("style","display:none");
                
                let card_body = document.create_element("div").expect("Error creating card body");
                card_body.set_class_name("card-body");
                
                /*
                ===========================
                    FOR EVERY FUNCTION/MAPPING CODE HERE
                ===========================
                */
                let sections = program.2.split("\n\n").collect::<Vec<&str>>();
                let mut section_index = 0;
                while section_index < sections.len(){
                    let section = sections[section_index];
                    let lines = section.split("\n").collect::<Vec<&str>>();
                    let section_re = Regex::new(r"(?<section_type>(function|finalize|mapping)) (?<section_name>.*):").unwrap();
                
                    match section_re.captures(lines[0]){
                        Some(caps) => {
                            let section_type = &caps["section_type"];
                            if section_type == "mapping"{
                                let mapping_name = &caps["section_name"];
                                let mapping_io_re = Regex::new(r"key as (?<type>.*)\.(?<visibility>.*);").unwrap();
                                match mapping_io_re.captures(lines[1]){
                                    Some(caps2) => {
                                        let full_type = format!("{}{}{}",&caps2["type"],".",&caps2["visibility"]);
                                        /*
                                        ===========================
                                            IF MAPPING CODE HERE
                                        ===========================
                                        */
                    
                                        /*
                                        ===========================
                                            MAPPING COMPRESSED
                                        ===========================
                                        */
                    
                                        let function_wrapper = document.create_element("div").expect("Error creating function wrapper");
                                        function_wrapper.set_class_name("function-wrapper");
                                        let _ = function_wrapper.set_attribute("program_name", &program.0);
                                        let _ = function_wrapper.set_attribute("name",&mapping_name);
                                        let _ = function_wrapper.set_attribute("function_type","mapping");
                    
                                        let input_field = document.create_element("div").expect("Error creating input field");
                                        input_field.set_class_name("input-field");
                    
                                        let output_input_wrapper = document.create_element("div").expect("Error creating output input wrapper");
                                        output_input_wrapper.set_class_name("output-input-wrapper");
                    
                                        let program_mapping_button = document.create_element("div").expect("Error creating program_function_button");
                                        program_mapping_button.set_class_name("program-mapping-button");
                                        let text = document.create_text_node(&mapping_name);
                                        let _ = program_mapping_button.append_child(&text);
                                        let _ = program_mapping_button.add_event_listener_with_callback("click", function_call.as_ref().unchecked_ref());
    
    
                                        let compressed_input = document.create_element("input").expect("Error creating compressed function input element");
                                        let _ = compressed_input.set_attribute("spellcheck", "false");
                                        let _ = compressed_input.set_attribute("autocomplete", "off");
                                        let _ = compressed_input.set_attribute("autocapitalize", "off");
                                        let _ = compressed_input.set_attribute("placeholder",&full_type);
                                        let _ = compressed_input.set_attribute("style","border-top-left-radius: 0px; border-bottom-left-radius: 0px; margin-right:5px");
                                        
                                        let function_img_expand = document.create_element("img").expect("Error creating function_img_expand element").dyn_into::<HtmlImageElement>().unwrap();
                                        let _ = function_img_expand.set_src("public/chevron-down.svg");
                                        function_img_expand.set_class_name("inactive");
                                        let _ = function_img_expand.set_attribute("style","cursor: pointer; order:1; z-index: 2;");
                                        let _ = function_img_expand.add_event_listener_with_callback("click", function_expand.as_ref().unchecked_ref());
                    
                    
                                        let _ = output_input_wrapper.append_child(&program_mapping_button);
                                        let _ = output_input_wrapper.append_child(&compressed_input);
                                        let _ = output_input_wrapper.append_child(&function_img_expand);
                                        let _ = input_field.append_child(&output_input_wrapper);
                    
                    
                                        /*
                                        ===========================
                                            MAPPING EXPANDED
                                        ===========================
                                        */
                    
                                        let function_expanded = document.create_element("div").expect("Error creating function_expanded");
                                        function_expanded.set_class_name("function-expanded");
                                        let _ = function_expanded.set_attribute("style","display:none");
                    
                                        let function_expanded_header = document.create_element("div").expect("Error creating function_expanded_header");
                                        function_expanded_header.set_class_name("function-expanded-header");
                    
                                        let function_expanded_title = document.create_element("div").expect("Error creating function_expanded_title");
                                        function_expanded_title.set_class_name("function-expanded-title");
                                        let text = document.create_text_node(&mapping_name);
                                        let _ = function_expanded_title.append_child(&text);
                                        let function_img_compress = document.create_element("img").expect("Error creating function_img_compress element").dyn_into::<HtmlImageElement>().unwrap();
                                        let _ = function_img_compress.set_src("public/chevron-up.svg");
                                        function_img_compress.set_class_name("inactive");
                                        let _ = function_img_compress.set_attribute("style","cursor: pointer; order:2; z-index: 2; padding-bottom:4px;");
                                        let _ = function_img_compress.add_event_listener_with_callback("click", function_compress.as_ref().unchecked_ref());
                    
                    
                                        let _ = function_expanded_header.append_child(&function_expanded_title);
                                        let _ = function_expanded_header.append_child(&function_img_compress);
                    
                                        let function_expanded_fields_wrapper = document.create_element("div").expect("Error creating function_expanded_fields_wrapper");
                                        function_expanded_fields_wrapper.set_class_name("function-expanded-fields-wrapper");
                    
                                        /*
                                        ===========================
                                            MAPPING KEY
                                        ===========================
                                        */
                    
                                        let function_expanded_field_wrapper = document.create_element("div").expect("Error creating function_expanded_field_wrapper");
                                        function_expanded_field_wrapper.set_class_name("function-expanded-field-wrapper");
                    
                                        let function_expanded_field_label = document.create_element("div").expect("Error creating function_expanded_field_label");
                                        function_expanded_field_label.set_class_name("function-expanded-field-label");
                                        let _ = function_expanded_field_label.set_attribute("style","padding-top:7px;");
                    
                                        let text = document.create_text_node("key: ");
                                        let _ = function_expanded_field_label.append_child(&text);
                                        let function_expanded_field = document.create_element("input").expect("Error creating compressed function_expanded_field");
                                        let _ = function_expanded_field.set_attribute("spellcheck", "false");
                                        let _ = function_expanded_field.set_attribute("autocomplete", "off");
                                        let _ = function_expanded_field.set_attribute("autocapitalize", "off");
                                        let _ = function_expanded_field.set_attribute("placeholder",&full_type);
                                        let _ = function_expanded_field.set_attribute("style","border-radius: 6px;");
                    
                                        let _ = function_expanded_field_wrapper.append_child(&function_expanded_field_label);
                                        let _ = function_expanded_field_wrapper.append_child(&function_expanded_field);
                                        let _ = function_expanded_fields_wrapper.append_child(&function_expanded_field_wrapper);
                    
                    
                                        /*
                                        ===========================
                                            FINALLY
                                        ===========================
                                        */
                    
                                        let function_expanded_submit_button_wrapper = document.create_element("div").expect("Error creating function_expanded_submit_button_wrapper");
                                        function_expanded_submit_button_wrapper.set_class_name("function-expanded-submit-button-wrapper");
                    
                                        let buffer2 = document.create_element("div").expect("Error creating function_expanded_execute_button buffer");
                                        let _ = buffer2.set_attribute("name","buffer");
                                        let _ = buffer2.set_attribute("style","width:100%");
                    
                                        let function_expanded_query_button = document.create_element("div").expect("Error creating function_expanded_query_button");
                                        function_expanded_query_button.set_class_name("function-expanded-query-button");
                    
                                        let text = document.create_text_node("Query");
                                        let _ = function_expanded_query_button.append_child(&text);
                                        let _ = function_expanded_query_button.add_event_listener_with_callback("click", function_call.as_ref().unchecked_ref());
    
                    
                    
                                        let _ = function_expanded_submit_button_wrapper.append_child(&buffer2);
                                        let _ = function_expanded_submit_button_wrapper.append_child(&function_expanded_query_button);
                    
                                        let _ = function_expanded.append_child(&function_expanded_header);
                                        let _ = function_expanded.append_child(&function_expanded_fields_wrapper);
                                        let _ = function_expanded.append_child(&function_expanded_submit_button_wrapper);
                    
                    
                    
                                        /*
                                        ===========================
                                            END MAPPING EXPANDED
                                        ===========================
                                        */
    
    
                                        let function_output = document.create_element("div").expect("Error creating function_output");
                                        let _ = function_output.set_attribute("class","function-output-title");
                                        let _ = function_output.set_attribute("style","display:none;");
    
                                        let function_error = document.create_element("div").expect("Error creating function_error");
                                        let _ = function_error.set_attribute("class","function-error-title");
                                        let _ = function_error.set_attribute("style","display:none;");
    
    
    
                    
                                        let _ = function_wrapper.append_child(&input_field);
                                        let _ = function_wrapper.append_child(&function_expanded);
                                        let _ = function_wrapper.append_child(&function_output);
                                        let _ = function_wrapper.append_child(&function_error);
    
                                    
    
                                        let _ = card_body.append_child(&function_wrapper);
                    
                                        /*
                                        ===========================
                                            END IF MAPPING CODE HERE
                                        ===========================
                                        */
                    
                                    }
                                    None => {}
                                }

                            } else if section_type == "function"{
                                let function_name = &caps["section_name"];
                                let input_re = Regex::new(r"input (?<input_number>r[0-9]*) as (?<type>.*)\.(?<visibility>.*);").unwrap(); 
                                let mut inputs : IndexMap<String,String> = IndexMap::new();
                                let mut one_line_types = String::new();
                                for function_line in &lines[1..]{
                                    match input_re.captures(function_line){
                                        Some(caps2) => {
                                            inputs.insert(caps2["input_number"].to_string(),format!("{}{}{}",&caps2["type"],".",&caps2["visibility"]));
                                            one_line_types = format!("{}{}{}{}{}",one_line_types, &caps2["type"],".",&caps2["visibility"],", ");
                                        }
                                        None => {
                                            break;
                                        }
                                    }
                                }
                                one_line_types.pop();
                                one_line_types.pop();
        
            
            
                                /*
                                ===========================
                                    IF FUNCTION CODE HERE
                                ===========================
                                */

                                let mut is_async = false;

                                if section_index != sections.len()-1 {
                                    let next_section = sections[section_index+ 1];
                                    let lines = next_section.split("\n").collect::<Vec<&str>>();
                                    let section_re = Regex::new(r"(?<section_type>finalize) (?<section_name>.*):").unwrap();
                                
                                    match section_re.captures(lines[0]){
                                        Some(_) => {
                                            is_async = true;
                                            section_index += 1;
                                        }
                                        None => {}
                                    }
                                }
            
                                /*
                                ===========================
                                    FUNCTION COMPRESSED
                                ===========================
                                */
            
                                let function_wrapper = document.create_element("div").expect("Error creating function wrapper");
                                function_wrapper.set_class_name("function-wrapper");
                                let _ = function_wrapper.set_attribute("name",&function_name);
                                let _ = function_wrapper.set_attribute("function_type",if is_async {"async-function"} else {"function"});
                                let _ = function_wrapper.set_attribute("program_name", &program.0);
            
                                let input_field = document.create_element("div").expect("Error creating input field");
                                input_field.set_class_name("input-field");
            
                                let output_input_wrapper = document.create_element("div").expect("Error creating output input wrapper");
                                output_input_wrapper.set_class_name("output-input-wrapper");
            
                                let program_function_button = document.create_element("div").expect("Error creating program_function_button");

                                program_function_button.set_class_name(if is_async {"program-async-function-button"} else {"program-function-button"});
                                let text = document.create_text_node(&function_name);
                                let _ = program_function_button.append_child(&text);
                                let _ = program_function_button.add_event_listener_with_callback("click", function_call.as_ref().unchecked_ref());

            
                                let compressed_input = document.create_element("input").expect("Error creating compressed function input element");
                                let _ = compressed_input.set_attribute("spellcheck", "false");
                                let _ = compressed_input.set_attribute("autocomplete", "off");
                                let _ = compressed_input.set_attribute("autocapitalize", "off");
                                let _ = compressed_input.set_attribute("placeholder",&one_line_types);
                                let _ = compressed_input.set_attribute("style","border-top-left-radius: 0px; border-bottom-left-radius: 0px; margin-right:5px");
                                
                                let function_img_expand = document.create_element("img").expect("Error creating function_img_expand element").dyn_into::<HtmlImageElement>().unwrap();
                                let _ = function_img_expand.set_src("public/chevron-down.svg");
                                function_img_expand.set_class_name("inactive");
                                let _ = function_img_expand.set_attribute("style","cursor: pointer; order:1; z-index: 2;");
                                let _ = function_img_expand.add_event_listener_with_callback("click", function_expand.as_ref().unchecked_ref());
            
            
                                let _ = output_input_wrapper.append_child(&program_function_button);
                                let _ = output_input_wrapper.append_child(&compressed_input);
                                let _ = output_input_wrapper.append_child(&function_img_expand);
                                let _ = input_field.append_child(&output_input_wrapper);
            
            
                                /*
                                ===========================
                                    FUNCTION EXPANDED
                                ===========================
                                */
            
                                let function_expanded = document.create_element("div").expect("Error creating function_expanded");
                                function_expanded.set_class_name("function-expanded");
                                let _ = function_expanded.set_attribute("style","display:none");
            
                                let function_expanded_header = document.create_element("div").expect("Error creating function_expanded_header");
                                function_expanded_header.set_class_name("function-expanded-header");
            
                                let function_expanded_title = document.create_element("div").expect("Error creating function_expanded_title");
                                function_expanded_title.set_class_name("function-expanded-title");
                                let text = document.create_text_node(&function_name);
                                let _ = function_expanded_title.append_child(&text);
                                let function_img_compress = document.create_element("img").expect("Error creating function_img_compress element").dyn_into::<HtmlImageElement>().unwrap();
                                let _ = function_img_compress.set_src("public/chevron-up.svg");
                                function_img_compress.set_class_name("inactive");
                                let _ = function_img_compress.set_attribute("style", "cursor: pointer; order:2; z-index: 2; padding-bottom:4px;");
                                let _ = function_img_compress.add_event_listener_with_callback("click", function_compress.as_ref().unchecked_ref());
            
            
                                let _ = function_expanded_header.append_child(&function_expanded_title);
                                let _ = function_expanded_header.append_child(&function_img_compress);
            
            
            
                                let function_expanded_fields_wrapper = document.create_element("div").expect("Error creating function_expanded_fields_wrapper");
                                function_expanded_fields_wrapper.set_class_name("function-expanded-fields-wrapper");
            
                                /*
                                ===========================
                                    FOR EVERY FIELD IN FUNCTION EXPANDED
                                ===========================
                                */
                                for (input_number, input_type) in inputs {
                                    let function_expanded_field_wrapper = document.create_element("div").expect("Error creating function_expanded_field_wrapper");
                                    function_expanded_field_wrapper.set_class_name("function-expanded-field-wrapper");
            
                                    let function_expanded_field_label = document.create_element("div").expect("Error creating function_expanded_field_label");
                                    function_expanded_field_label.set_class_name("function-expanded-field-label");
                                    let text = document.create_text_node(&format!("{}{}", input_number, ": "));
                                    let _ = function_expanded_field_label.append_child(&text);
                                    let function_expanded_field = document.create_element("input").expect("Error creating compressed function_expanded_field");
                                    let _ = function_expanded_field.set_attribute("spellcheck", "false");
                                    let _ = function_expanded_field.set_attribute("autocomplete", "off");
                                    let _ = function_expanded_field.set_attribute("autocapitalize", "off");
                                    let _ = function_expanded_field.set_attribute("placeholder",&input_type);
                                    let _ = function_expanded_field.set_attribute("style","border-radius: 6px;");
            
                                    let _ = function_expanded_field_wrapper.append_child(&function_expanded_field_label);
                                    let _ = function_expanded_field_wrapper.append_child(&function_expanded_field);
                                    let _ = function_expanded_fields_wrapper.append_child(&function_expanded_field_wrapper);
            
                                }
                                /*
                                ===========================
                                    FINALLY
                                ===========================
                                */
            
                                let function_expanded_submit_button_wrapper = document.create_element("div").expect("Error creating function_expanded_submit_button_wrapper");
                                function_expanded_submit_button_wrapper.set_class_name("function-expanded-submit-button-wrapper");
            
                                let buffer2 = document.create_element("div").expect("Error creating function_expanded_execute_button buffer");
                                let _ = buffer2.set_attribute("name","buffer");
                                let _ = buffer2.set_attribute("style","width:100%");
            
                                let function_expanded_execute_button = document.create_element("div").expect("Error creating function_expanded_execute_button");
                                function_expanded_execute_button.set_class_name(if is_async {"function-expanded-async-execute-button"} else {"function-expanded-execute-button"});
                                let text = document.create_text_node("Execute");
                                let _ = function_expanded_execute_button.append_child(&text);
                                let _ = function_expanded_execute_button.add_event_listener_with_callback("click", function_call.as_ref().unchecked_ref());

            
            
                                let _ = function_expanded_submit_button_wrapper.append_child(&buffer2);
                                let _ = function_expanded_submit_button_wrapper.append_child(&function_expanded_execute_button);
            
                                let _ = function_expanded.append_child(&function_expanded_header);
                                let _ = function_expanded.append_child(&function_expanded_fields_wrapper);
                                let _ = function_expanded.append_child(&function_expanded_submit_button_wrapper);
            
            
                                /*
                                ===========================
                                    END FUNCTION EXPANDED
                                ===========================
                                */
                
                                let function_output = document.create_element("div").expect("Error creating function_output");
                                let _ = function_output.set_attribute("class","function-output-title");
                                let _ = function_output.set_attribute("style","display:none;");

                                let function_error = document.create_element("div").expect("Error creating function_error");
                                let _ = function_error.set_attribute("class","function-error-title");
                                let _ = function_error.set_attribute("style","display:none;");



            
                                let _ = function_wrapper.append_child(&input_field);
                                let _ = function_wrapper.append_child(&function_expanded);
                                let _ = function_wrapper.append_child(&function_output);
                                let _ = function_wrapper.append_child(&function_error);
                                let _ = card_body.append_child(&function_wrapper);
            
                                /*
                                ===========================
                                    END IF FUNCTION CODE HERE
                                ===========================
                                */
                
                            }
                        }
                        None => {}
                    }
                    section_index += 1;
                }
                let _ = card_body_wrapper.append_child(&card_body);
                let _ = program_card.append_child(&program_custom_head);
                let _ = program_card.append_child(&card_body_wrapper);
                
                let programs = document.query_selector(".programs-wrapper").unwrap().unwrap();
                let _ = programs.append_child(&program_card);
            }
        }
    });


    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */


    Effect::new({
        move || {
            let network_item = current_environment_dropdown_item.get();
            if network_item == "devnet-button" {
                set_network_accounts.set(accounts.get().0);
            } else {
                set_network_accounts.set(accounts.get().1);
            }
        }
    });

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#deploy-execute-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">
                Deploy and Execute
            </div>

            <div class="sidebar-body-wrapper" style="overflow:auto;">
                <div id="funding-card" style="color:#e3e3e3;" class="card">
                    <div id="funding-card-head" class="card-head" >
                        <div class="title" style="-webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none;">
                            Funding
                        </div>
                    </div>
                    <div class="card-body-wrapper">
                        <div id="funding-card-body" class="card-body">
                            <div class="input-field"  style="color:#e3e3e3;">
                                <div class="field-title">{move || format!("{}{}","Network: ",current_environment_dropdown_text.get())}</div>
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
                                        <div id="placeholder-button" class="dropdown-item-placeholder" style={move || if network_accounts.get().len() == 0 {"display: block; border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {"display: none; border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"}}
                                        >
                                            Please load an account first!
                                        </div>
                                        <For each=move || network_accounts.get() key=|(key,_)| key.to_string() children=move |(name,_)| {
                                            view! {
                                                <div id=name class={ let name_clone = name.clone(); move || { let id = deploy_accounts_dropdown_item.get(); if id == name_clone  {"dropdown-item selected"} else {"dropdown-item"}}} style={ let name_clone = name.clone(); move || { let accounts_map = network_accounts.get(); if accounts_map.len() != 0 {let final_item = &accounts_map.get_index(accounts_map.len()-1).unwrap(); if final_item.0.to_string() == name_clone {"border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {""}} else {""}}}
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

                                        let private_fee_input = document.query_selector("#private-fee-input").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        if value {
                                            let _ = private_fee_input.set_attribute("active", "true");
                                        } else  {
                                            let _ = private_fee_input.set_attribute("active", "false");
                                        }           
                                    }
                                    />
                                    <span class="slider round"></span>
                                </label>
                            </div>


                            <div class="input-field" id="private-fee-input-field" style="padding-top:10px; display: none;">
                                <div style="order:0" class="field-title">Fee Record</div>

                                <div class="output-textarea-wrapper" style="height:70px;">
                                    <textarea style="order:0; white-space: pre-wrap; background-color:transparent;" id="private-fee-input" placeholder="Fee Record" spellcheck="false" autocomplete="off" autocapitalize="off" active="false"/>
                                </div>
                            </div>
                        </div>
                    </div>
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
                            <div class="field-title">{move || format!("{}{}","Network: ",current_environment_dropdown_text.get())}</div>
                            
                            // <div class="input-field">
                            //     <div class="field-title">Program ID</div>
                            //     <input id="deploy-input-program-id" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            // </div>


                            <div class="input-field">
                                <div class="field-title">Project</div>
                                <div class="output-input-wrapper">
                                    <input id="deploy-input-project" value={move || compiled_project.get().1} placeholder="Compile a project first!" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                </div>
                            </div>
                            <div class="input-field">
                                <div class="field-title">Program ID</div>
                                <div class="output-input-wrapper">
                                    <input id="deploy-input-program-id" value={move || compiled_program_id.get()} placeholder="Compile a project first!" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                </div>
                            </div>
                            <div id="deploy-program-error" class="error-title" style="display:none; padding-top:0px; padding-left:2px;"></div>

                        </div>
                        <div class="card-divider"/>

                        <button id="deploy-button" class="card-button"
                        on:click:target=move|ev| {
                            let document = leptos::prelude::document();
                                                        
                            let current_project_input = document.query_selector("#deploy-input-project").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_program_id_input = document.query_selector("#deploy-input-program-id").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_fee_input = document.query_selector("#deploy-input-fee").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_fee_record_input = document.query_selector("#private-fee-input").unwrap().unwrap().dyn_into::<HtmlTextAreaElement>().unwrap();

                            let project = current_project_input.value().clone();
                            let program_id = current_program_id_input.value().clone();
                            let fee = current_fee_input.value().clone();
                            let fee_record = current_fee_record_input.value().clone();
                            
                

                            let target1 = current_project_input.dyn_into::<HtmlElement>().unwrap();
                            let target2 = current_program_id_input.dyn_into::<HtmlElement>().unwrap();
                            let target3 = current_fee_input.dyn_into::<HtmlElement>().unwrap();
                            let target4 = document.query_selector("#deploy-accounts-dropdown-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let target5 = current_fee_record_input.dyn_into::<HtmlElement>().unwrap();
                            let private_fee_active = target5.get_attribute("active").unwrap();

                            let style1 = target1.style();
                            let style2 = target2.style();
                            let style3 = target3.style();
                            let style4 = target4.style();
                            let style5 = target5.style();

                            if &project == "" {
                                let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style1.set_property("border", "1px solid #494e64");   
                            }

                            if &program_id == "" {
                                let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style2.set_property("border", "1px solid #494e64");   
                            }

                            if &fee == "" {
                                let _ = style3.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style3.set_property("border", "1px solid #494e64");   
                            }

                            if deploy_accounts_dropdown_item.get_untracked() == String::new(){
                                let _ = style4.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style4.set_property("border", "1px solid #494e64");   
                            }
                
                            if private_fee_active == "true" && &fee_record == ""{
                                let _ = style5.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style5.set_property("border", "1px solid #494e64");   
                            }

                            if &project != "" && &program_id != "" && &fee != "" && deploy_accounts_dropdown_item.get_untracked() != String::new() && !(private_fee_active == "true" && &fee_record == ""){
                                let _ = style1.set_property("border", "1px solid #494e64");  
                                let _ = style2.set_property("border", "1px solid #494e64");  
                                let _ = style3.set_property("border", "1px solid #494e64");  
                                let _ = style4.set_property("border", "1px solid #494e64");
                                let _ = style5.set_property("border", "1px solid #494e64");
                                let error = document.query_selector("#deploy-program-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none"); 

                                let this = ev.target().dyn_into::<Element>().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                                let _ = this.class_list().add(&new_val); 


                                spawn_local(async move {
                                    let current_env = if current_environment_dropdown_text.get_untracked() == "Local Devnet" {"local".to_string()} else {current_environment_dropdown_text.get_untracked().to_string().to_lowercase()};
                                    let network : String = if current_environment_dropdown_item.get_untracked() == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};
                                    
                                    let accounts_map = network_accounts.get_untracked();
                                    let account = accounts_map.get(&deploy_accounts_dropdown_item.get_untracked()).unwrap();
                                    let pk = account.0.clone();
                                    let fee_in_microcredits : u64; //* 1000000;
                                    match fee.parse::<u64>() {
                                        Ok(val) => {fee_in_microcredits = val *1000000},
                                        Err(_) => {fee_in_microcredits = 0},
                                    }

                                    let mut command = vec!["deploy".to_string(), "--path".to_string(),compiled_project.get_untracked().0.clone(), "--yes".to_string(), "--no-build".to_string(), "--private-key".to_string(), pk,"--base-fee".to_string(),fee_in_microcredits.to_string(),"--network".to_string(),network.clone(),"--endpoint".to_string(),current_endpoint.get_untracked()];
                                    if private_fee_active == "true" {
                                        command.push("--record".to_string());
                                        command.push(fee_record);
                                    }
                                    
                                    
                                    let args = serde_wasm_bindgen::to_value(&Command { command : command}).unwrap();        
                                    let (error,_output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                    if !error {
                                        let compiled_file = format!("{}{}",compiled_project.get_untracked().0.clone(),"/build/main.aleo");
                                        let args = serde_wasm_bindgen::to_value(&ReadFileArgs{filepath: compiled_file}).unwrap();
                                        match invoke("read_file", args).await.as_string(){
                                            Some(contents) => {
                                                let mut formatted_output = String::new();
                                                let split = contents.split("\n").collect::<Vec<&str>>();
                                                for item in &(split)[2..split.len()]{
                                                    if *item == "" {
                                                        formatted_output = format!("{}{}", formatted_output, "\n");
                                                    } else {
                                                        formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                                    }
                                                }
                                                set_new_loaded_program.set((program_id, current_env.clone(), formatted_output));
                                            },
                                            None => {
                                                console_log("Error: File does not exist");
                                            }
                                        }
                                    } else {
                                        let error = document.query_selector("#deploy-program-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: Deployment Failed");
                                        let _ = error.style().set_property("display", "block");

                                    }
                                    let _ = this.class_list().remove(&new_val); 
                                })
                            }
            
                        }
                        >
                            Deploy
                        </button>
                    </div>

                    <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-program-button" {"display: flex"} else {"display: none"}}>
                        <div id="load-program-card-body" class="card-body">
                            <div class="field-title">{move || format!("{}{}","Network: ",current_environment_dropdown_text.get())}</div>
                            <div class="input-field">
                                <div class="field-title">Program ID</div>
                                <input id="load-program-input" placeholder="Program ID" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                                <div id="load-program-input-error" class="error-title" style="display:none;"></div>
                            </div>
                        </div>
                        <div class="card-divider"/>
                        <button id="load-program-button" class="card-button"
                        on:click:target=move|ev| {
                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#load-program-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let value = current_input.value().clone();
                            let current_env = if current_environment_dropdown_text.get_untracked() == "Local Devnet" {"local".to_string()} else {current_environment_dropdown_text.get_untracked().to_string().to_lowercase()};

                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            if &value == "" {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style.set_property("border", "1px solid #494e64");
                                let error = document.query_selector("#load-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");

                                
                                let this = ev.target().dyn_into::<Element>().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                                let _ = this.class_list().add(&new_val); 

                                let formatted_id = format!("{}{}{}{}",value," (",current_env,")");
                                let element_option = document.get_element_by_id(&formatted_id);
                                match element_option {
                                    Some(_) =>{
                                        let error = document.query_selector("#load-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error.set_inner_html("Error: This program has already been loaded for the selected network.");
                                        let _ = error.style().set_property("display", "block");
                                        let _ = this.class_list().remove(&new_val); 
                                    },
                                    None => {
                                        /*
                                        Dependency Checking Order:
                                            1. TODO: Local
                                                a. Look in ROOT/program.json
                                                b. If exists, will be at path ROOT/program.json["path"]/build/main.aleo
                                            2. Query
                                                a. Will look in .aleo/NETWORK if already cached
                                                b. Else, will query from network
                                        */

                                        //let mut not_local = true;
                                        spawn_local(async move {
                                            // if compiled_project.get_untracked().0 != String::new() {
                                            // if false {
                                            //     let args = serde_wasm_bindgen::to_value(&ReadProgramJsonArgs { filepath : format!("{}{}", compiled_project.get_untracked().0, "/program.json"), field: "dependencies".to_string()}).unwrap();
                                            //     let dependencies = invoke("read_program_json", args).await.as_string().unwrap();

                                            //     if dependencies != String::new(){
                                            //         let deserialized_return_val : Vec<HashMap<String,Option<String>>> = serde_json::from_str(&dependencies).expect("Error with decoding dir_entry");
                                            //         for json in deserialized_return_val {
                                            //             let name = json.get("name").unwrap().clone().unwrap();
                                            //             if name == value{
                                            //                 let location = json.get("location").unwrap().clone().unwrap();
                                            //                 if location == "local" {
                                            //                     let path = json.get("path").unwrap().clone().unwrap();
                                            //                     let full_path = format!("{}{}{}{}", compiled_project.get_untracked().0, "/", path,"/build/main.aleo");
                                            //                     let args = serde_wasm_bindgen::to_value(&ReadFileArgs{filepath: full_path}).unwrap();
                                            //                     match invoke("read_file", args).await.as_string(){
                                            //                         Some(contents) => {
                                            //                             console_log(&contents);
                                            //                         },
                                            //                         None => {
                                            //                             console_log("Error: File does not exist");
                                            //                         }
                                            //                     }
                                                                
                                                                
                                            //                     not_local = false;
                                            //                     break;
                                            //                 }
                                            //             }
                                            //         }
                                            //     }
                                            // }
                                            if true {//not_local {
                                                let network : String = if current_environment_dropdown_item.get_untracked() == "mainnet-button" {"mainnet".to_string()} else {"testnet".to_string()};

                                                let args = serde_wasm_bindgen::to_value(&Command { command : vec!["query".to_string(),"program".to_string(), value.clone() ,"--network".to_string(),network.clone(),"--endpoint".to_string(),current_endpoint.get_untracked()]}).unwrap();        
                                                let (error,output): (bool, String) = serde_wasm_bindgen::from_value(invoke("execute", args).await).unwrap();
                                                if !error {
                                                    let mut formatted_output = String::new();
                                                    let split = output.split("\n\n").collect::<Vec<&str>>();
                                                    for item in &(split)[2..split.len()-3]{
                                                        if *item == "" {
                                                            formatted_output = format!("{}{}", formatted_output, "\n");
                                                        } else {
                                                            formatted_output = format!("{}{}{}", formatted_output, item, "\n");
                                                        }
                                                    }

                                                    set_new_loaded_program.set((value.clone(), current_env.clone(), formatted_output));
                                                    
                                                } else {
                                                    let error = document.query_selector("#load-program-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                                    error.set_inner_html("Error: The program with this ID does not exist.");
                                                    let _ = error.style().set_property("display", "block");
                                                }
                                            }
                                            let _ = this.class_list().remove(&new_val); 
                                        });
                                    }
                                }
                            }
                        }
                        >
                            Load
                        </button>
                    </div>
                </div>


                <div class="panel-divider" style="margin-bottom:0"/>

                <div class="programs-wrapper"></div>
            </div>
        </div>
    }
}