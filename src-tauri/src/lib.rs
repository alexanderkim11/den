// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod clipboard;
mod warning;
mod file;
mod highlight;
mod leo;
mod load_theme_syntax;
mod open_explorer;
mod snarkvm;
mod url;
mod test;
mod state;

use tauri::{Builder, Manager};
use tauri_plugin_shell::process::CommandChild;

use std::sync::Mutex;
// use indexmap::IndexMap;
// use tauri_plugin_store::StoreExt;



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


#[derive(Default)]
struct AppState {
    amareleo: Option<CommandChild>,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));
            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested{..} => {
                let app_handle = window.app_handle();
                let state = app_handle.state::<Mutex<AppState>>();
                let mut state = state.lock().unwrap();
                let amareleo = state.amareleo.take();
                match amareleo{
                    Some(process) => {
                        let _ = process.kill();
                    }
                    None => {}
                }
                // let exit : bool = dialog::exit_warning(app_handle.clone(),"null".to_string());
                // if !exit {
                //     api.prevent_close();
                // }
            }
            _ => {}
        })
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            clipboard::copy,
            warning::warning,
            warning::exit_warning,
            warning::change_dir_warning,
            file::read_file,
            file::write_file,
            file::mkdir,
            file::read_program_json,
            file::path_exists,
            highlight::highlight,
            leo::execute,
            load_theme_syntax::load_leo_syntax,
            load_theme_syntax::load_aleo_syntax,
            open_explorer::open_explorer,
            open_explorer::get_directory,
            snarkvm::new_account,
            snarkvm::account_from_pk,
            snarkvm::address_from_vk,
            snarkvm::sign,
            snarkvm::verify,
            snarkvm::decrypt_record,
            snarkvm::execute_remote_wrapper,
            state::get_state_accounts,
            state::get_state_root_dir,
            state::update_state_accounts,
            state::update_state_root_dir,
            state:: start_dev_node,
            url::open_url,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


