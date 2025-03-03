use indexmap::IndexMap;

use std::env::current_dir;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_os::*;
use tauri::Manager;


use tauri_plugin_store::StoreExt;
use serde_json::{json, Value};

use crate::AppState;
use std::sync::Mutex;


#[tauri::command]
pub fn get_state_accounts(handle: tauri::AppHandle, _placeholder : String) -> (IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>) {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    let option = state.get("accounts");
    match option {
        Some(accounts_json) => {
            let saved_accounts : IndexMap<String,(String,String,String)>;
            let mut dev_accounts : IndexMap<String,(String,String,String)>;
            let val = &accounts_json["saved_accounts"];
            match val{
                Value::Null => {
                    saved_accounts = IndexMap::new();
                },
                _ => {
                    saved_accounts = serde_json::from_value(val.clone()).unwrap();

                }
            }

            let val2 = &accounts_json["dev_accounts"];
            match val2{
                Value::Null => {
                    dev_accounts = IndexMap::new();
                    dev_accounts.insert("Account 0".to_string(),("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string(),"AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw".to_string(),"aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px".to_string()));
                    dev_accounts.insert("Account 1".to_string(),("APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh".to_string(),"AViewKey1pTzjTxeAYuDpACpz2k72xQoVXvfY4bJHrjeAQp6Ywe5g".to_string(),"aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t".to_string()));
                    dev_accounts.insert("Account 2".to_string(),("APrivateKey1zkp2GUmKbVsuc1NSj28pa1WTQuZaK5f1DQJAT6vPcHyWokG".to_string(),"AViewKey1u2X98p6HDbsv36ZQRL3RgxgiqYFr4dFzciMiZCB3MY7A".to_string(),"aleo1ashyu96tjwe63u0gtnnv8z5lhapdu4l5pjsl2kha7fv7hvz2eqxs5dz0rg".to_string()));
                    dev_accounts.insert("Account 3".to_string(),("APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs".to_string(),"AViewKey1iKKSsdnatHcm27goNC7SJxhqQrma1zkq91dfwBdxiADq".to_string(),"aleo12ux3gdauck0v60westgcpqj7v8rrcr3v346e4jtq04q7kkt22czsh808v2".to_string()));
                
                },
                _ => {
                    dev_accounts = serde_json::from_value(val2.clone()).unwrap();

                }
            }


            return (dev_accounts,saved_accounts)
        },
        None => {
            let mut dev_accounts = IndexMap::new();
            dev_accounts.insert("Account 0".to_string(),("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string(),"AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw".to_string(),"aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px".to_string()));
            dev_accounts.insert("Account 1".to_string(),("APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh".to_string(),"AViewKey1pTzjTxeAYuDpACpz2k72xQoVXvfY4bJHrjeAQp6Ywe5g".to_string(),"aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t".to_string()));
            dev_accounts.insert("Account 2".to_string(),("APrivateKey1zkp2GUmKbVsuc1NSj28pa1WTQuZaK5f1DQJAT6vPcHyWokG".to_string(),"AViewKey1u2X98p6HDbsv36ZQRL3RgxgiqYFr4dFzciMiZCB3MY7A".to_string(),"aleo1ashyu96tjwe63u0gtnnv8z5lhapdu4l5pjsl2kha7fv7hvz2eqxs5dz0rg".to_string()));
            dev_accounts.insert("Account 3".to_string(),("APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs".to_string(),"AViewKey1iKKSsdnatHcm27goNC7SJxhqQrma1zkq91dfwBdxiADq".to_string(),"aleo12ux3gdauck0v60westgcpqj7v8rrcr3v346e4jtq04q7kkt22czsh808v2".to_string()));
        
            state.set("accounts", json!({"dev_accounts": json!(dev_accounts)}));
            return (dev_accounts,IndexMap::new())
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
pub fn update_state_accounts(handle: tauri::AppHandle, updatedAccounts : (IndexMap<String,(String,String,String)>,IndexMap<String,(String,String,String)>)) {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    // let mut dev_accounts : IndexMap<String,(String,String,String)> = IndexMap::new();
    // dev_accounts.insert("Account 0".to_string(),("APrivateKey1zkp8CZNn3yeCseEtxuVPbDCwSyhGW6yZKUYKfgXmcpoGPWH".to_string(),"AViewKey1mSnpFFC8Mj4fXbK5YiWgZ3mjiV8CxA79bYNa8ymUpTrw".to_string(),"aleo1rhgdu77hgyqd3xjj8ucu3jj9r2krwz6mnzyd80gncr5fxcwlh5rsvzp9px".to_string()));
    // dev_accounts.insert("Account 1".to_string(),("APrivateKey1zkp2RWGDcde3efb89rjhME1VYA8QMxcxep5DShNBR6n8Yjh".to_string(),"AViewKey1pTzjTxeAYuDpACpz2k72xQoVXvfY4bJHrjeAQp6Ywe5g".to_string(),"aleo1s3ws5tra87fjycnjrwsjcrnw2qxr8jfqqdugnf0xzqqw29q9m5pqem2u4t".to_string()));
    // dev_accounts.insert("Account 2".to_string(),("APrivateKey1zkp2GUmKbVsuc1NSj28pa1WTQuZaK5f1DQJAT6vPcHyWokG".to_string(),"AViewKey1u2X98p6HDbsv36ZQRL3RgxgiqYFr4dFzciMiZCB3MY7A".to_string(),"aleo1ashyu96tjwe63u0gtnnv8z5lhapdu4l5pjsl2kha7fv7hvz2eqxs5dz0rg".to_string()));
    // dev_accounts.insert("Account 3".to_string(),("APrivateKey1zkpBjpEgLo4arVUkQmcLdKQMiAKGaHAQVVwmF8HQby8vdYs".to_string(),"AViewKey1iKKSsdnatHcm27goNC7SJxhqQrma1zkq91dfwBdxiADq".to_string(),"aleo12ux3gdauck0v60westgcpqj7v8rrcr3v346e4jtq04q7kkt22czsh808v2".to_string()));


    let accounts = state.get("accounts");
    match accounts {
        Some(mut val) => {
            val["dev_accounts"] = json!(updatedAccounts.0);
            val["saved_accounts"] = json!(updatedAccounts.1);
            state.set("accounts", json!(val));
        }
        None => {
            state.set("accounts", json!({"dev_accounts": json!(updatedAccounts.0),"saved_accounts":json!(updatedAccounts.1)}));
        }
    }

    let _  = state.save();
}

#[tauri::command]
pub fn update_state_root_dir(handle: tauri::AppHandle, directory : String) {
    let state = handle.store("C:\\Users\\r0ami\\Home\\state.json").expect("Error loading state!");
    //let state = handle.store("./").expect("Error loading state!");

    state.set("root_dir", json!(directory));

    let _  = state.save();
}


#[tauri::command]
pub async fn start_dev_node(handle: tauri::AppHandle, _placeholder : String) {
    let current_dir = current_dir().unwrap();
    let platform = platform();
    let bin = if platform == "windows" {"amareleo-chain-x86_64-pc-windows-msvc.exe"} else if platform == "macos" { if arch() == "aarch64" {"amareleo-chain-aarch64-apple-darwin"} else {"amareleo-chain-x86_64-apple-darwin"} } else {"amareleo-chain-x86_64-unknown-linux-gnu"};
    let exec_bin = format!(
        "{}{}{}",
        current_dir.as_path().to_str().unwrap(),
        "/.amareleo/",
        bin
    );
    let shell = handle.shell();

    let output = shell
        .command(&exec_bin)
        .args(["clean"])
        .output()
        .await
        .unwrap();


    if output.status.success() {
        let (_, child) = shell
            .command(exec_bin)
            .args(["start"])
            .spawn()
            .expect("Failed to spawn amareleo");


        let state = handle.state::<Mutex<AppState>>();

        // Lock the mutex to get mutable access:
        let mut state = state.lock().unwrap();

        //Save child to state
        state.amareleo = Some(child);
    }
    


}
