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
mod state;

use std::sync::Mutex;
use indexmap::IndexMap;
use tauri::{Builder, Manager};
use tauri_plugin_store::StoreExt;



// // Define the plugin config
// #[derive(Default)]
// struct AppState {
//     root_dir: String,
//     accounts:IndexMap<String,(String,String,String)>,
// //   open_files: Vec<(String,String)>,
// //   selected_file : String,
// //   cached_files: HashMap<String,String>,
// //   saved_files: HashMap<String,String>,
// }

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            // This loads the store from disk
            //let store = app.store("state.json")?;
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
            file::mkdir,
            file::read_program_json,
            highlight::highlight,
            leo::execute,
            load_theme_syntax::load,
            open_explorer::open_explorer,
            open_explorer::get_directory,
            snarkvm::new_account,
            snarkvm::account_from_pk,
            snarkvm::address_from_vk,
            snarkvm::sign,
            snarkvm::verify,
            snarkvm::decrypt_record,
            state::get_state_accounts,
            state::get_state_root_dir,
            state::update_state_accounts,
            state::update_state_root_dir,
            test::test,
            url::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


