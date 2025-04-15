use leptos::web_sys::{Element,HtmlElement,HtmlInputElement, HtmlImageElement, HtmlTextAreaElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use indexmap::IndexMap;
use regex::Regex;
use leptos::ev::Event;


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


    let transaction_compress_expand = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let card = this.parent_element().unwrap().parent_element().unwrap().parent_element().unwrap();  
        let img = this.dyn_into::<HtmlImageElement>().unwrap();
        let content = card.children().item(1).unwrap().dyn_into::<HtmlElement>().unwrap();
        if img.class_name() == "inactive"{ //Expand
            img.set_src("public/chevron-down.svg");
            img.set_class_name("active");
            card.set_class_name("transaction-card active");
            let _ = content.set_attribute("style","display: flex; border:0;");
        } else { //Compress
            img.set_src("public/chevron-right.svg");
            img.set_class_name("inactive");
            card.set_class_name("transaction-card");   
            let _ = content.set_attribute("style","display:none;");
        }
    }) as Box<dyn FnMut(_)>);
    
    let transaction_close = Closure::wrap(Box::new(move |ev: Event| {
        let this = ev.target().unwrap().dyn_into::<Element>().unwrap();
        let card = this.parent_element().unwrap().parent_element().unwrap().parent_element().unwrap();  
        card.remove();
    }) as Box<dyn FnMut(_)>);



    
    Effect::new({
        move || {
            if true {
                let document = leptos::prelude::document();
                /*
                ===========================
                    TRANSACTION START 
                ===========================
                */
                let transaction_card = document.create_element("div").expect("Error creating transaction card");
                transaction_card.set_class_name("transaction-card");
                let transaction_title = "at1mz48z8sasetazmpw8juwtr95l5xwq3esa8eu2lywnuegxsqv9crs5ca4h6";
                transaction_card.set_id(&transaction_title);
                
                /*
                ===========================
                    HEADER CODE HERE
                ===========================
                */
                
                let transaction_custom_head = document.create_element("div").expect("Error creating transaction custom head");
                transaction_custom_head.set_class_name("transaction-custom-head");
                
                let transaction_dropdown_button = document.create_element("div").expect("Error creating transaction dropdown button");
                transaction_dropdown_button.set_class_name("dropdown-button");

                let header_img_status = document.create_element("img").expect("Error creating header img expand element").dyn_into::<HtmlImageElement>().expect("Error casting into HTMLImageElement");
                let _ = header_img_status.set_src("public/circle-filled-dark.svg");
                header_img_status.set_class_name("transaction-pending");
                let _ = header_img_status.set_attribute("style","order:0; z-index: 2;");

                
                let header_img_expand = document.create_element("img").expect("Error creating header img expand element").dyn_into::<HtmlImageElement>().expect("Error casting into HTMLImageElement");
                let _ = header_img_expand.set_src("public/chevron-right.svg");
                header_img_expand.set_class_name("inactive");
                let _ = header_img_expand.set_attribute("style","order:1; z-index: 2;");
                let _ = header_img_expand.add_event_listener_with_callback("click", transaction_compress_expand.as_ref().unchecked_ref());
                
                
                let buffer = document.create_element("div").expect("Error creating transaction header buffer element");
                buffer.set_class_name("buffer");
                let _ = buffer.set_attribute("style", "order:2; overflow:hidden; margin-right:10px;");
                let text = document.create_text_node(&transaction_title);
                let _ = buffer.append_child(&text);
                
                let header_img_close = document.create_element("img").expect("Error creating header img close element").dyn_into::<HtmlImageElement>().unwrap();
                let _ = header_img_close.set_src("public/close.svg");
                header_img_close.set_class_name("inactive");
                let _ = header_img_close.set_attribute("style","order:3; z-index: 2;");
                let _ = header_img_close.add_event_listener_with_callback("click", transaction_close.as_ref().unchecked_ref());
                
                let _ = transaction_dropdown_button.append_child(&header_img_status);
                let _ = transaction_dropdown_button.append_child(&header_img_expand);
                let _ = transaction_dropdown_button.append_child(&buffer);
                let _ = transaction_dropdown_button.append_child(&header_img_close);
                let _ = transaction_custom_head.append_child(&transaction_dropdown_button);
                
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
                
                let transaction_id = document.create_element("div").expect("Error creating card body");
                transaction_id.set_class_name("transaction-details");
                let transaction_id_text = document.create_text_node("Transaction ID: at1mz48z8sasetazmpw8juwtr95l5xwq3esa8eu2lywnuegxsqv9crs5ca4h6");
                let _ = transaction_id.append_child(&transaction_id_text);
                let _ = card_body.append_child(&transaction_id);

                let transaction_timestamp = document.create_element("div").expect("Error creating card body");
                transaction_timestamp.set_class_name("transaction-details");
                let transaction_timestamp_text = document.create_text_node("Timestamp: 4/15/2025 9:31:39 AM");
                let _ = transaction_timestamp.append_child(&transaction_timestamp_text);
                let _ = card_body.append_child(&transaction_timestamp);

                let transaction_type = document.create_element("div").expect("Error creating card body");
                transaction_type.set_class_name("transaction-details");
                let transaction_type_text = document.create_text_node("Type: Deployment");
                let _ = transaction_type.append_child(&transaction_type_text);
                let _ = card_body.append_child(&transaction_type);

                let transaction_program = document.create_element("div").expect("Error creating card body");
                transaction_program.set_class_name("transaction-details");
                let transaction_program_text = document.create_text_node("Program: credits.aleo");
                let _ = transaction_program.append_child(&transaction_program_text);
                let _ = card_body.append_child(&transaction_program);

                let transaction_function= document.create_element("div").expect("Error creating card body");
                transaction_function.set_class_name("transaction-details");
                let transaction_function_text = document.create_text_node("[Optional] Function: transfer_public");
                let _ = transaction_function.append_child(&transaction_function_text);
                let _ = card_body.append_child(&transaction_function);

                let transaction_function_inputs = document.create_element("div").expect("Error creating card body");
                transaction_function_inputs.set_class_name("transaction-details");
                let transaction_function_inputs_text = document.create_text_node("[Optional] Inputs: 1u8, 2u8");
                let _ = transaction_function_inputs.append_child(&transaction_function_inputs_text);
                let _ = card_body.append_child(&transaction_function_inputs);

                let transaction_status= document.create_element("div").expect("Error creating card body");
                transaction_status.set_class_name("transaction-details");
                let transaction_status_text = document.create_text_node("Status: Pending");
                let _ = transaction_status.append_child(&transaction_status_text);
                let _ = card_body.append_child(&transaction_status);

                let _ = card_body_wrapper.append_child(&card_body);
                let _ = transaction_card.append_child(&transaction_custom_head);
                let _ = transaction_card.append_child(&card_body_wrapper);
                
                let transactions = document.query_selector(".transactions-wrapper").unwrap().unwrap();
                let _ = transactions.append_child(&transaction_card);
            }
        }
    });




    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#history-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">History</div>

            <div class="transactions-wrapper"></div>
        </div>
    }
}
