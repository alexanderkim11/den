use indexmap::IndexMap;

use tauri_plugin_store::StoreExt;
use serde_json::{json, Value};


#[tauri::command]
pub fn get_state_accounts(handle: tauri::AppHandle, _placeholder : String) -> IndexMap<String,(String,String,String)> {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    let option = state.get("accounts");
    match option {
        Some(accounts_json) => {
            let accounts : IndexMap<String,(String,String,String)> = serde_json::from_value(accounts_json).unwrap();
            return accounts
        },
        None => {
            let empty : IndexMap<String,(String,String,String)> = IndexMap::new();
            return empty
        }
    }

}

#[tauri::command]
pub fn get_state_root_dir(handle: tauri::AppHandle, _placeholder : String) -> Value {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    let option = state.get("root_dir");
    match option {
        Some(dir) => return dir,
        None => return serde_json::Value::String(String::new())
    }

}


#[tauri::command]
pub fn update_state_accounts(handle: tauri::AppHandle, updatedAccounts : IndexMap<String,(String,String,String)>) {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    state.set("accounts", json!(updatedAccounts));

    let _  = state.save();
}

#[tauri::command]
pub fn update_state_root_dir(handle: tauri::AppHandle, directory : String) {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    state.set("root_dir", json!(directory));

    let _  = state.save();
}
