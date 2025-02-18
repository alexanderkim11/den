// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod clipboard;
mod dialog;
mod file;
mod highlight;
mod leo;
mod load_theme_syntax;
mod open_explorer;
mod snarkvm;
mod url;
mod test;

use std::{collections::HashMap, sync::Mutex};
use indexmap::IndexMap;
use tauri::{Builder, Manager, RunEvent};



// Define the plugin config
#[derive(Default)]
struct AppState {
  open_files: Vec<(String,String)>,
  selected_file : String,
  cached_files: HashMap<String,String>,
  saved_files: HashMap<String,String>, 
  accounts:IndexMap<String,(String,String,String)>,

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested{api,..} => {
                let app_handle = window.app_handle();
                let exit : bool = dialog::exit_warning(app_handle.clone(),"null".to_string());
                if !exit {
                    api.prevent_close();
                }
            }
            _ => {}
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            clipboard::copy,
            dialog::warning,
            dialog::exit_warning,
            file::read_file,
            file::write_file,
            highlight::highlight,
            leo::execute,
            load_theme_syntax::load,
            open_explorer::open_explorer,
            snarkvm::new_account,
            snarkvm::account_from_pk,
            snarkvm::address_from_vk,
            snarkvm::sign,
            snarkvm::verify,
            snarkvm::decrypt_record,
            test::test,
            url::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


