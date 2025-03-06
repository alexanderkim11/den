use leptos::web_sys::{Element,HtmlElement,HtmlInputElement};
use js_sys::Array;
use leptos::{leptos_dom::logging::console_log, task::spawn_local};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use indexmap::IndexMap;

use crate::app::CopyButton;

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
pub struct NewAccountArgs<> {
    pub network : String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountFromPrivateKeyArgs<> {
    pub network : String,
    pub privatekey : String
}

#[derive(Serialize, Deserialize)]
pub struct AddressFromViewKeyArgs<> {
    pub network : String,
    pub viewkey : String
}

#[derive(Serialize, Deserialize)]
pub struct SignMessageArgs<> {
    pub network : String,
    pub privatekey: String,
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyMessageArgs<> {
    pub network : String,
    pub address: String,
    pub message: String,
    pub signature: String
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStateAccountsArgs<> {
    pub updatedAccounts : (IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>),
}

/*
==============================================================================
COMPONENTS
==============================================================================
*/

#[component]
pub fn SidebarAccount (
    selected_activity_icon: ReadSignal<String>,
    accounts : ReadSignal<(IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>)>,
    set_accounts : WriteSignal<(IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>)>,
    current_environment_dropdown_item : ReadSignal<String>,
) -> impl IntoView {

    /*
    ==============================================================================
    REACTIVE SIGNALS
    ==============================================================================
    */

    let (dropdown_active, set_dropdown_active) = signal(false);
    let (current_dropdown_item, set_current_dropdown_item) = signal("saved-accounts-button".to_string());
    let (current_dropdown_text, set_current_dropdown_text) = signal("Saved Accounts".to_string());


    //TODO: edit the starting values if accounts are loaded from cache
    let (saved_accounts_dropdown_active, set_saved_accounts_dropdown_active) = signal(false);
    let (saved_accounts_dropdown_item, set_saved_accounts_dropdown_item) = signal(String::new());
    let (saved_accounts_dropdown_text, set_saved_accounts_dropdown_text) = signal("--".to_string());

    let (network_accounts, set_network_accounts) = signal(IndexMap::new());

    /*
    ==============================================================================
    HELPER FUNCTIONS
    ==============================================================================
    */

    fn account_check(new_entry :(&str,&str,&str), accounts : &IndexMap<String,(String,String,String)>) -> bool{
        let new_pk = new_entry.0;
        let new_vk = new_entry.1;
        let new_address = new_entry.2;
        for (_,(pk,vk,address)) in accounts {
            if new_pk == &pk.to_string() && new_vk == &vk.to_string() && new_address == &address.to_string() {
                return true;
            }
        }
        return false;
    }


    

    /*
    ==============================================================================
    MAIN VIEW
    ==============================================================================
    */
    

    Effect::new({
        move || {
            let network_item = current_environment_dropdown_item.get();
            set_saved_accounts_dropdown_active.set(false);
            set_saved_accounts_dropdown_item.set(String::new());
            set_saved_accounts_dropdown_text.set("--".to_string());
            if network_item == "devnet-button" {
                set_network_accounts.set(accounts.get().0);
            } else {
                set_network_accounts.set(accounts.get().1);
            }
        }
    });

    // let network_item = current_environment_dropdown_item.get_untracked();
    // let network : String = if network_item == "mainnet-button" {"mainnet".to_string()} else if network_item == "testnet-button" {"testnet".to_string()} else {"devnet".to_string()};

    view! {
        <div class="wrapper" style={move || if selected_activity_icon.get() == "#account-tab-button" {"display: flex;"} else {"display: none;"}}>
            <div class="sidebar-title">Accounts</div>
            <div id="account-card" class="card">

                <div id="account-dropdown-custom" class="dropdown-custom-head" style="z-index:3">
                    <div id="account-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
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
                    <div id="account-dropdown-content" class="dropdown-content" style={move || if dropdown_active.get() {"display: block"} else {"display: none"}}>

                        <div id="saved-accounts-button" class={move || if current_dropdown_item.get() == "saved-accounts-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Saved Accounts
                        </div>

                        <div id="create-new-account-button" class={move || if current_dropdown_item.get() == "create-new-account-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Create a New Account
                        </div>
                        <div id="load-account-from-pk-button" class={move || if current_dropdown_item.get() == "load-account-from-pk-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Load Account from Private Key
                        </div>
                        <div id="load-address-from-vk-button" class={move || if current_dropdown_item.get() == "load-address-from-vk-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());
                                
                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Load Address from View Key
                        </div>
                        <div id="sign-message-button" class={move || if current_dropdown_item.get() == "sign-message-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Sign a Message
                        </div>
                        <div id="verify-message-button" style="border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;" class={move || if current_dropdown_item.get() == "verify-message-button" {"dropdown-item selected"} else {"dropdown-item"}}
                        on:click:target = move|ev| {
                            if current_dropdown_item.get() != ev.target().id(){
                                set_current_dropdown_item.set(ev.target().id());
                                set_current_dropdown_text.set(ev.target().inner_html());

                                let document = leptos::prelude::document();
                                let target = document.query_selector("#account-dropdown-button").unwrap().unwrap();
                                let new_val = Array::new();
                                new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                let _ = target.class_list().remove(&new_val);
                                set_dropdown_active.set(false);
                            }
                        }
                        >
                            Verify a Message
                        </div>
                    </div>

                </div>


                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "saved-accounts-button" {"display: flex"} else {"display: none"}}>
                    <div id="saved-accounts-card-body" class="card-body">

                        <div class="input-field"  style="color:#e3e3e3;">
                            <div class="field-title">Account</div>
                            <div id="saved-accounts-dropdown-custom" class="dropdown-custom">
                                <div id="saved-accounts-dropdown-button" class="dropdown-button" on:click:target=move|ev| 
                                {
                                    let this = ev.target().dyn_into::<Element>().unwrap();
                                    let new_val = Array::new();
                                    new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                    if this.class_list().contains("show"){
                                        let _ = this.class_list().remove(&new_val);
                                        set_saved_accounts_dropdown_active.set(false);
                                    } else {
                                        let _ = this.class_list().add(&new_val);
                                        set_saved_accounts_dropdown_active.set(true);
                                    }
                                }> 
                                    <div class="buffer" inner_html={move || saved_accounts_dropdown_text.get()}></div>
                                    <img src="public/chevron-down.svg"/>
                                </div>
                                <div id="saved-accounts-dropdown-content" class="dropdown-content" style={move || if saved_accounts_dropdown_active.get() {"display: block"} else {"display: none"}}>
                                    <div id="placeholder-button" class="dropdown-item-placeholder" style={move || if network_accounts.get().len() == 0 {"display: block; border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {"display: none; border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"}}
                                    >
                                        Please load an account first!
                                    </div>
                                    <For each=move || network_accounts.get() key=|(key,_)| key.to_string() children=move |(name,_)| {
                                        view! {
                                            <div id=name class={ let name_clone = name.clone(); move || { let id = saved_accounts_dropdown_item.get(); if id == name_clone  {"dropdown-item selected"} else {"dropdown-item"}}} style={ let name_clone = name.clone(); move || { let accounts_map = network_accounts.get(); if accounts_map.len() != 0 {let final_item = &accounts_map.get_index(accounts_map.len()-1).unwrap(); if final_item.0.to_string() == name_clone {"border-bottom-left-radius: 6px; border-bottom-right-radius: 6px;"} else {""}} else {""}}}
                                            on:click:target = move|ev| {
                                                let current_item = saved_accounts_dropdown_item.get();
                                                if current_item != ev.target().id(){
                                                    set_saved_accounts_dropdown_item.set(ev.target().id());
                                                    set_saved_accounts_dropdown_text.set(ev.target().inner_html());
                    
                                                    let document = leptos::prelude::document();
                                                    let target = document.query_selector("#saved-accounts-dropdown-button").unwrap().unwrap();
                                                    let new_val = Array::new();
                                                    new_val.push(&serde_wasm_bindgen::to_value("show").unwrap());
                                                    let _ = target.class_list().remove(&new_val);
                                                    set_saved_accounts_dropdown_active.set(false);

                                                    let map = network_accounts.get();
                                                    let (pk,vk,address) = map.get(&saved_accounts_dropdown_item.get()).unwrap();
                            
                                                    let document = leptos::prelude::document();
                            
                                                    let pk_output_element = document.query_selector("#saved-accounts-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                    let vk_output_element = document.query_selector("#saved-accounts-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                                    let address_output_element = document.query_selector("#saved-accounts-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            
                                                    pk_output_element.set_value(&pk);
                                                    vk_output_element.set_value(&vk);
                                                    address_output_element.set_value(&address);
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

                        <div class="output-field" style={move || if saved_accounts_dropdown_item.get() == String::new() {"display:none"} else {"display: block"}}>
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="saved-accounts-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#saved-accounts-output-pk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field" style={move || if saved_accounts_dropdown_item.get() == String::new() {"display:none"} else {"display: block"}}>
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="saved-accounts-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#saved-accounts-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field" style={move || if saved_accounts_dropdown_item.get() == String::new() {"display:none"} else {"display: block"}}>
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="saved-accounts-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#saved-accounts-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="card-divider"  style={move || if saved_accounts_dropdown_item.get() == String::new() {"display:none"} else {"display: block"}}/>

                    <button id="saved-accounts-delete-button" class="card-button-delete" style={move || if saved_accounts_dropdown_item.get() == String::new() {"display:none"} else {"display: inline-block"}}
                    on:click:target=move|_| {
                        let name = saved_accounts_dropdown_item.get();
                        set_saved_accounts_dropdown_item.set(String::new());
                        set_saved_accounts_dropdown_text.set("--".to_string());
                        set_saved_accounts_dropdown_active.set(false);

                        let document = leptos::prelude::document();

                        let pk_output_element = document.query_selector("#saved-accounts-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let vk_output_element = document.query_selector("#saved-accounts-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let address_output_element = document.query_selector("#saved-accounts-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                        pk_output_element.set_value("");
                        vk_output_element.set_value("");
                        address_output_element.set_value("");

                        let mut untracked_accounts = network_accounts.get_untracked();
                        untracked_accounts.shift_remove(&name);
                        set_network_accounts.set(untracked_accounts);

                        let network_item = current_environment_dropdown_item.get_untracked();
                        let current_accounts = accounts.get_untracked();
                        if network_item == "devnet-button" {
                            set_accounts.set((network_accounts.get_untracked(),current_accounts.1));
                        } else {
                            set_accounts.set((current_accounts.0,network_accounts.get_untracked()));
                        }

                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&UpdateStateAccountsArgs { updatedAccounts: accounts.get_untracked()}).unwrap();
                            invoke("update_state_accounts", args).await;
                        });
                    }
                    >
                        Delete
                    </button>
                </div>


                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "create-new-account-button" {"display: flex"} else {"display: none"}}>
                    <div id="create-account-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Name</div>
                            <input id="create-new-account-input-name" placeholder="Account Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="create-new-account-error" class="error-title" style="display:none;"></div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="create-new-account-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#create-new-account-output-pk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="create-new-account-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#create-new-account-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="create-new-account-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#create-new-account-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="create-new-account-generate-button" class="card-button"
                    on:click:target=move|ev| {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                        let _ = this.class_list().add(&new_val);
    
                        let document = leptos::prelude::document();

                        spawn_local(async move {
                            let args = serde_wasm_bindgen::to_value(&NewAccountArgs {network : "Mainnet".to_string()}).unwrap();
                            let (error, (pk,vk,address)): (bool, (String,String,String)) = serde_wasm_bindgen::from_value(invoke("new_account", args).await).unwrap();

                            if !error {
                                let pk_output_element = document.query_selector("#create-new-account-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let vk_output_element = document.query_selector("#create-new-account-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                let address_output_element = document.query_selector("#create-new-account-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                                pk_output_element.set_value(&pk);
                                vk_output_element.set_value(&vk);
                                address_output_element.set_value(&address);

                                let old_button = document.query_selector("#create-new-account-generate-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let new_button = document.query_selector("#create-new-account-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                
                                let _ = old_button.style().set_property("display", "none");
                                let _ = new_button.style().set_property("display", "flex");

                                let _ = this.class_list().remove(&new_val);
                            } else {
                                let _ = this.class_list().remove(&new_val);
                            }
                        });
                    }
                    >
                        Generate
                    </button>
                    <div id="create-new-account-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                        <button id="create-new-account-save-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();
                            let current_input = document.query_selector("#create-new-account-input-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let name = current_input.value();
                            let target = current_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();
                            let mut untracked_accounts = network_accounts.get_untracked();

                            let pk_output_element = document.query_selector("#create-new-account-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_output_element = document.query_selector("#create-new-account-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let address_output_element = document.query_selector("#create-new-account-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                            let pk = pk_output_element.value();
                            let vk = vk_output_element.value();
                            let address = address_output_element.value();

                            if &name == "" {
                                let error = document.query_selector("#create-new-account-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none"); 
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");   
                            } else if untracked_accounts.contains_key(&name) {
                                let error = document.query_selector("#create-new-account-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                error.set_inner_html("Error: This account name is already taken");
                                let _ = error.style().set_property("display", "block");
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");
                            } else if account_check((&pk,&vk,&address),&untracked_accounts) {
                                let error = document.query_selector("#create-new-account-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                error.set_inner_html("Error: This account has already been saved");
                                let _ = error.style().set_property("display", "block");
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");      
                            } else {
                                let error = document.query_selector("#create-new-account-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                let _ = style.set_property("border", "1px solid #494e64"); 

                                untracked_accounts.insert(name,(pk,vk,address));
                                set_network_accounts.set(untracked_accounts);

                                let network_item = current_environment_dropdown_item.get_untracked();
                                let current_accounts = accounts.get_untracked();
                                if network_item == "devnet-button" {
                                    set_accounts.set((network_accounts.get_untracked(),current_accounts.1));
                                } else {
                                    set_accounts.set((current_accounts.0,network_accounts.get_untracked()));
                                }
        

                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&UpdateStateAccountsArgs { updatedAccounts: accounts.get_untracked()}).unwrap();
                                    invoke("update_state_accounts", args).await;
                                });
                            }
                        }
                        >
                            Save
                        </button>
                        <button id="create-new-account-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();

                            let input_style = document.query_selector("#create-new-account-input-name").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap().style();
                            let _ = input_style.set_property("border", "1px solid #494e64");
                            let error = document.query_selector("#create-new-account-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error.style().set_property("display", "none");

                            let new_button = document.query_selector("#create-new-account-generate-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_button = document.query_selector("#create-new-account-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            
                            let _ = old_button.style().set_property("display", "none");
                            let _ = new_button.style().set_property("display", "inline-block");    

                            let pk_output_element = document.query_selector("#create-new-account-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_output_element = document.query_selector("#create-new-account-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let address_output_element = document.query_selector("#create-new-account-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let name_input = document.query_selector("#create-new-account-input-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            
                            name_input.set_value("");                    
                            pk_output_element.set_value("");
                            vk_output_element.set_value("");
                            address_output_element.set_value("");
                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-account-from-pk-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-account-from-pk-input-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Private Key</div>
                            <input id="load-account-from-pk-input" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="load-account-from-pk-input-error" class="error-title" style="display:none;"></div>
                        </div>
                    </div>
                    <div id="load-account-from-pk-output-card-body" class="card-body" style="display:none;">
                        <div class="input-field">
                            <div class="field-title">Name</div>
                            <input id="load-account-from-pk-output-name" placeholder="Account Name" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="load-account-from-pk-output-error" class="error-title" style="display:none;"></div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="load-account-from-pk-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-account-from-pk-output-pk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="load-account-from-pk-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-account-from-pk-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="load-account-from-pk-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-account-from-pk-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-account-from-pk-load-button" class="card-button"
                    on:click:target=move|ev| {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                        let _ = this.class_list().add(&new_val);

                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#load-account-from-pk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = (&current_input).value();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");
                            let _ = this.class_list().remove(&new_val);   
                            let error_element = document.query_selector("#load-account-from-pk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error_element.style().set_property("display", "none");
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");
                            let error_element = document.query_selector("#load-account-from-pk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error_element.style().set_property("display", "none");

                            spawn_local(async move {
                                let args = serde_wasm_bindgen::to_value(&AccountFromPrivateKeyArgs {network : "Mainnet".to_string(), privatekey : value.clone()}).unwrap();
                                let (error, (pk,vk,address)): (bool, (String,String,String)) = serde_wasm_bindgen::from_value(invoke("account_from_pk", args).await).unwrap();


                                if !error {
                                    //Reset pk input so it doesn't  remain
                                    let current_input = document.query_selector("#load-account-from-pk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    current_input.set_value("");

                                    let pk_output_element = document.query_selector("#load-account-from-pk-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let vk_output_element = document.query_selector("#load-account-from-pk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let address_output_element = document.query_selector("#load-account-from-pk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                                    pk_output_element.set_value(&pk);
                                    vk_output_element.set_value(&vk);
                                    address_output_element.set_value(&address);

                                    let old_body = document.query_selector("#load-account-from-pk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_body = document.query_selector("#load-account-from-pk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let old_button = document.query_selector("#load-account-from-pk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_button = document.query_selector("#load-account-from-pk-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    
                                    let _ = old_body.style().set_property("display", "none");
                                    let _ = old_button.style().set_property("display", "none");
                                    let _ = new_body.style().set_property("display", "block");
                                    let _ = new_button.style().set_property("display", "flex");          
    
                                    let _ = this.class_list().remove(&new_val);
                                } else {
                                    let error_element = document.query_selector("#load-account-from-pk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    error_element.set_inner_html("Error: Invalid private key");
                                    let _ = error_element.style().set_property("display", "block");
                                    let _ = this.class_list().remove(&new_val);
                                }
                            });   
                        }
                    }
                    >
                        Load
                    </button>
                    <div id="load-account-from-pk-double-button" class="double-button-wrapper" style="order:3; display:none; justify-content:center">
                        <button id="load-account-from-pk-save-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();
                            let current_name_input = document.query_selector("#load-account-from-pk-output-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let name = current_name_input.value();
                            let target = current_name_input.dyn_into::<HtmlElement>().unwrap();
                            let style = target.style();

                            let mut untracked_accounts = network_accounts.get_untracked();

                            let pk_output_element = document.query_selector("#load-account-from-pk-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_output_element = document.query_selector("#load-account-from-pk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let address_output_element = document.query_selector("#load-account-from-pk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                            let pk = pk_output_element.value();
                            let vk = vk_output_element.value();
                            let address = address_output_element.value();

                            if name == "".to_string() {
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");
                            } else if untracked_accounts.contains_key(&name) {
                                let error = document.query_selector("#load-account-from-pk-output-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                error.set_inner_html("Error: This account name is already taken");
                                let _ = error.style().set_property("display", "block");
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");
                            } else if account_check((&pk,&vk,&address),&untracked_accounts) {
                                let error = document.query_selector("#load-account-from-pk-output-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                error.set_inner_html("Error: This account has already been saved");
                                let _ = error.style().set_property("display", "block");
                                let _ = style.set_property("border", "1px solid var(--grapefruit)");         
                            } else {
                                let error = document.query_selector("#load-account-from-pk-output-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = error.style().set_property("display", "none");
                                let _ = style.set_property("border", "1px solid #494e64");

                                untracked_accounts.insert(name,(pk,vk,address));
                                set_network_accounts.set(untracked_accounts);

                                let network_item = current_environment_dropdown_item.get_untracked();
                                let current_accounts = accounts.get_untracked();
                                if network_item == "devnet-button" {
                                    set_accounts.set((network_accounts.get_untracked(),current_accounts.1));
                                } else {
                                    set_accounts.set((current_accounts.0,network_accounts.get_untracked()));
                                }
        

                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&UpdateStateAccountsArgs { updatedAccounts: accounts.get_untracked()}).unwrap();
                                    invoke("update_state_accounts", args).await;
                                });
                            }
                        }
                        >
                            Save
                        </button>
                        <button id="load-account-from-pk-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();

                            let new_body = document.query_selector("#load-account-from-pk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_body = document.query_selector("#load-account-from-pk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let new_button = document.query_selector("#load-account-from-pk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_button = document.query_selector("#load-account-from-pk-double-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            
                            let _ = old_body.style().set_property("display", "none");
                            let _ = old_button.style().set_property("display", "none");
                            let _ = new_body.style().set_property("display", "block");
                            let _ = new_button.style().set_property("display", "inline-block");    

                            let pk_output_element = document.query_selector("#load-account-from-pk-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let vk_output_element = document.query_selector("#load-account-from-pk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let address_output_element = document.query_selector("#load-account-from-pk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let name_output_element = document.query_selector("#load-account-from-pk-output-name").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            
                            name_output_element.set_value("");            
                            pk_output_element.set_value("");
                            vk_output_element.set_value("");
                            address_output_element.set_value("");

                            let error = document.query_selector("#load-account-from-pk-output-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error.style().set_property("display", "none");
                            let _ = name_output_element.dyn_into::<HtmlElement>().unwrap().style().set_property("border", "1px solid #494e64"); 
                        }
                        >
                            Clear
                        </button>
                    </div>

                </div>


                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "load-address-from-vk-button" {"display: flex"} else {"display: none"}}>
                    <div id="load-address-from-vk-input-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">View Key</div>
                            <input id="load-address-from-vk-input" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                            <div id="load-address-from-vk-input-error" class="error-title" style="display:none;"></div>
                        </div>
                    </div>

                    <div id="load-address-from-vk-output-card-body" class="card-body" style="display:none;">
                        <div class="output-field">
                            <div class="field-title">View Key</div>
                            <div class="output-input-wrapper">
                                <input id="load-address-from-vk-output-vk" placeholder="View Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-address-from-vk-output-vk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Address</div>
                            <div class="output-input-wrapper">
                                <input id="load-address-from-vk-output-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#load-address-from-vk-output-address".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>
                    <div class="card-divider"/>
                    <button id="load-address-from-vk-load-button" class="card-button"
                    on:click:target=move|ev| {
                        let this = ev.target().dyn_into::<Element>().unwrap();
                        let new_val = Array::new();
                        new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                        let _ = this.class_list().add(&new_val);

                        let document = leptos::prelude::document();
                        let current_input = document.query_selector("#load-address-from-vk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let value = (&current_input).value();
                        let target = current_input.dyn_into::<HtmlElement>().unwrap();
                        let style = target.style();
                        if &value == "" {
                            let _ = style.set_property("border", "1px solid var(--grapefruit)");
                            let _ = this.class_list().remove(&new_val);
                            let error_element = document.query_selector("#load-address-from-vk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error_element.style().set_property("display", "none");     
                        } else {
                            let _ = style.set_property("border", "1px solid #494e64");
                            let error_element = document.query_selector("#load-address-from-vk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = error_element.style().set_property("display", "none");
    
                            spawn_local(async move {
                                let args = serde_wasm_bindgen::to_value(&AddressFromViewKeyArgs {network : "Mainnet".to_string(), viewkey : value.clone()}).unwrap();
                                let (error, address): (bool, String) = serde_wasm_bindgen::from_value(invoke("address_from_vk", args).await).unwrap();
                                
                                if !error{
                                    //Reset vk input so it doesn't get remain
                                    let current_input = document.query_selector("#load-address-from-vk-input").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    current_input.set_value("");

                                    let vk_output_element = document.query_selector("#load-address-from-vk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let address_output_element = document.query_selector("#load-address-from-vk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                                    vk_output_element.set_value(&value);
                                    address_output_element.set_value(&address);

                                    let old_body = document.query_selector("#load-address-from-vk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_body = document.query_selector("#load-address-from-vk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let old_button = document.query_selector("#load-address-from-vk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    let new_button = document.query_selector("#load-address-from-vk-clear-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    
                                    let _ = old_body.style().set_property("display", "none");
                                    let _ = old_button.style().set_property("display", "none");
                                    let _ = new_body.style().set_property("display", "block");
                                    let _ = new_button.style().set_property("display", "inline-block");

                                    let _ = this.class_list().remove(&new_val);    
                                } else {
                                    let error_element = document.query_selector("#load-address-from-vk-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                    error_element.set_inner_html("Error: Invalid private key");
                                    let _ = error_element.style().set_property("display", "block");
                                    let _ = this.class_list().remove(&new_val);

                                }
                            });   
                        }
                    }
                    >
                        Load
                    </button>
                    <button id="load-address-from-vk-clear-button" class="card-button-clear" style="display:none;"
                    on:click:target=move|_ev| {
                        let document = leptos::prelude::document();
    
                        let new_body = document.query_selector("#load-address-from-vk-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let old_body = document.query_selector("#load-address-from-vk-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let new_button = document.query_selector("#load-address-from-vk-load-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        let old_button = document.query_selector("#load-address-from-vk-clear-button").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                        
                        let _ = old_body.style().set_property("display", "none");
                        let _ = old_button.style().set_property("display", "none");
                        let _ = new_body.style().set_property("display", "block");
                        let _ = new_button.style().set_property("display", "inline-block");    
    
                        let vk_output_element = document.query_selector("#load-address-from-vk-output-vk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        let address_output_element = document.query_selector("#load-address-from-vk-output-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                        
                        vk_output_element.set_value("");
                        address_output_element.set_value("");
                    }
                    >
                        Clear
                    </button>
                </div>





                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "sign-message-button" {"display: flex"} else {"display: none"}}>
                    <div id="sign-message-input-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Private Key</div>
                            <input id="sign-message-input-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field">
                            <div class="field-title">Message</div>
                            <input id="sign-message-input-message" placeholder="Message" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Signature</div>
                            <div class="output-input-wrapper">
                                <input id="sign-message-output" placeholder="Signature" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#sign-message-output".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                            <div id="sign-message-input-error" class="error-title" style="display:none;"></div>
                        </div>
                    </div>
                    <div id="sign-message-output-card-body" class="card-body" style="display:none;">
                        <div class="output-field">
                            <div class="field-title">Private Key</div>
                            <div class="output-input-wrapper">
                                <input id="sign-message-output-pk" placeholder="Private Key" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#sign-message-output-pk".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Message</div>
                            <div class="output-input-wrapper">
                                <input id="sign-message-output-message" placeholder="Message" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#sign-message-output-message".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                        <div class="output-field">
                            <div class="field-title">Signature</div>
                            <div class="output-input-wrapper">
                                <input id="sign-message-output-signature" placeholder="Signature" spellcheck="false" autocomplete="off" autocapitalize="off" readonly/>
                                <div class="output-img-wrapper">
                                    <CopyButton target_field="#sign-message-output-signature".to_string() element_type="Input".to_string()/>
                                </div>
                            </div>
                        </div>
                    </div>

                    <div class="card-divider"/>
                    <div class="double-button-wrapper" style="order:3; display:flex; justify-content:center">
                        <button id="sign-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);

                            let document = leptos::prelude::document();
                            
                            let current_pk_input = document.query_selector("#sign-message-input-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_message_input = document.query_selector("#sign-message-input-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                            let pk = current_pk_input.value();
                            let message = current_message_input.value();
    
                            let target1 = current_pk_input.dyn_into::<HtmlElement>().unwrap();
                            let target2 = current_message_input.dyn_into::<HtmlElement>().unwrap();
    
                            let style1 = target1.style();
                            let style2 = target2.style();
    
                            if &pk == "" {
                                let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style1.set_property("border", "1px solid #494e64");   
                            }
    
                            if &message == "" {
                                let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style2.set_property("border", "1px solid #494e64");   
                            }


                            if &message != "" && &pk != ""{
                                let _ = style1.set_property("border", "1px solid #494e64");   
                                let _ = style2.set_property("border", "1px solid #494e64");  

                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&SignMessageArgs {network : "Mainnet".to_string(), privatekey : pk.clone(), message: message.clone()}).unwrap();
                                    let (error, signature): (bool, String) = serde_wasm_bindgen::from_value(invoke("sign", args).await).unwrap();

                                    // //Reset pk and message inputs so it doesn't get remain
                                    let current_pk_input = document.query_selector("#sign-message-input-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                    let current_message_input = document.query_selector("#sign-message-input-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
            
                                    current_pk_input.set_value("");
                                    current_message_input.set_value("");

                                    if !error {
                                        let pk_output_element = document.query_selector("#sign-message-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                        let message_output_element = document.query_selector("#sign-message-output-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                                        let signature_output_element = document.query_selector("#sign-message-output-signature").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
        
                                        pk_output_element.set_value(&pk);
                                        message_output_element.set_value(&message);
                                        signature_output_element.set_value(&signature);

                                        let old_body = document.query_selector("#sign-message-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        let new_body = document.query_selector("#sign-message-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        
                                        let _ = old_body.style().set_property("display", "none");
                                        let _ = new_body.style().set_property("display", "block");

                                        let _ = this.class_list().remove(&new_val);      
        
                                    } else {
                                        let error_element = document.query_selector("#sign-message-input-error").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        error_element.set_inner_html("Error: Could not generate signature");
                                        let _ = error_element.style().set_property("display", "block");
                                        let _ = this.class_list().remove(&new_val);
                                    }
                                });   
                            } else {
                                let _ = this.class_list().remove(&new_val);
                            }
                        }
                        >
                            Sign
                        </button>
                        <button id="signature-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();
    
                            let new_body = document.query_selector("#sign-message-input-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let old_body = document.query_selector("#sign-message-output-card-body").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            
                            let _ = old_body.style().set_property("display", "none");
                            let _ = new_body.style().set_property("display", "block");

                            let pk_output_element = document.query_selector("#sign-message-output-pk").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let message_output_element = document.query_selector("#sign-message-output-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let signature_output_element = document.query_selector("#sign-message-output-signature").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                            pk_output_element.set_value("");
                            message_output_element.set_value("");
                            signature_output_element.set_value("");
                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>

                <div class="card-body-wrapper" style={move || if current_dropdown_item.get() == "verify-message-button" {"display: flex"} else {"display: none"}}>
                    <div id="verify-message-card-body" class="card-body">
                        <div class="input-field">
                            <div class="field-title">Address</div>
                            <input id="verify-message-input-address" placeholder="Address" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field">
                            <div class="field-title">Message</div>
                            <input id="verify-message-input-message" placeholder="Message" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                        <div class="input-field">
                            <div class="field-title">Signature</div>
                            <input id="verify-message-input-signature" placeholder="Signature" spellcheck="false" autocomplete="off" autocapitalize="off"/>
                        </div>
                    </div>

                    <div class="card-divider"/>

                    <div id="verify-message-output-message"  style="display:none; order:3; padding-bottom: 10px; padding-left: 10px; font-size: 12px; font-weight: 400; font-family: 'Inter'; -webkit-user-select: none; -khtml-user-select: none; -moz-user-select: none; -ms-user-select: none; user-select: none"></div>

                    <div class="double-button-wrapper" style="order:4; display:flex; justify-content:center">
                        <button id="verify-button" class="card-button" style="margin-right:10px;"
                        on:click:target=move|ev| {
                            let this = ev.target().dyn_into::<Element>().unwrap();
                            let new_val = Array::new();
                            new_val.push(&serde_wasm_bindgen::to_value("pending").unwrap());
                            let _ = this.class_list().add(&new_val);
                            

                            let document = leptos::prelude::document();

                            let output_element = document.query_selector("#verify-message-output-message").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            let _ = output_element.style().set_property("display", "none"); 

                            let current_address_input = document.query_selector("#verify-message-input-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_message_input = document.query_selector("#verify-message-input-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let current_signature_input = document.query_selector("#verify-message-input-signature").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
    
                            let address = current_address_input.value();
                            let message = current_message_input.value();
                            let signature = current_signature_input.value();
    
    
                            let target1 = current_address_input.dyn_into::<HtmlElement>().unwrap();
                            let target2 = current_message_input.dyn_into::<HtmlElement>().unwrap();
                            let target3 = current_signature_input.dyn_into::<HtmlElement>().unwrap();
    
    
                            let style1 = target1.style();
                            let style2 = target2.style();
                            let style3 = target3.style();
    
                            if &address == "" {
                                let _ = style1.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style1.set_property("border", "1px solid #494e64");   
                            }
    
                            if &message == "" {
                                let _ = style2.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style2.set_property("border", "1px solid #494e64");   
                            }
    
                            if &signature == "" {
                                let _ = style3.set_property("border", "1px solid var(--grapefruit)");   
                            } else {
                                let _ = style3.set_property("border", "1px solid #494e64");   
                            }


                            if &address != "" && &message != "" && &signature != ""{
                                let _ = style1.set_property("border", "1px solid #494e64");   
                                let _ = style2.set_property("border", "1px solid #494e64");
                                let _ = style3.set_property("border", "1px solid #494e64");
                                let output_element = document.query_selector("#verify-message-output-message").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                let _ = output_element.style().set_property("display", "block"); 

                                spawn_local(async move {
                                    let args = serde_wasm_bindgen::to_value(&VerifyMessageArgs {network : "Mainnet".to_string(), address : address.clone(), message: message.clone(), signature: signature.clone()}).unwrap();
                                    let (error, message): (bool, String) = serde_wasm_bindgen::from_value(invoke("verify", args).await).unwrap();


                                    if !error {
                                        let output_element = document.query_selector("#verify-message-output-message").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        output_element.set_inner_html(&message);
                                        let _ = output_element.style().set_property("color", "var(--lime)");
                                        let _ = output_element.style().set_property("display", "block");
        
                                    } else {
                                        let output_element = document.query_selector("#verify-message-output-message").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                                        output_element.set_inner_html(&message);
                                        let _ = output_element.style().set_property("color", "var(--grapefruit)");
                                        let _ = output_element.style().set_property("display", "block");
                                    }

                                    let _ = this.class_list().remove(&new_val);
                                });   
                            } else {
                                let _ = this.class_list().remove(&new_val);
                            }
                        }
                        >
                            Verify
                        </button>
                        <button id="signature-clear-button" class="card-button-clear" style="margin-left:10px;"
                        on:click:target=move|_ev| {
                            let document = leptos::prelude::document();

                            let pk_output_element = document.query_selector("#verify-message-input-address").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let message_output_element = document.query_selector("#verify-message-input-message").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();
                            let signature_output_element = document.query_selector("#verify-message-input-signature").unwrap().unwrap().dyn_into::<HtmlInputElement>().unwrap();

                            pk_output_element.set_value("");
                            message_output_element.set_value("");
                            signature_output_element.set_value("");

                            let output_element = document.query_selector("#verify-message-output-message").unwrap().unwrap().dyn_into::<HtmlElement>().unwrap();
                            output_element.set_inner_html("");
                            let _ = output_element.style().remove_property("color");
                            let _ = output_element.style().set_property("display", "none");
                        }
                        >
                            Clear
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}