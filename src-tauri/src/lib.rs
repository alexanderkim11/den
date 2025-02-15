// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod highlight;
mod leo;
mod load_theme_syntax;
mod open_explorer;
mod file;
mod clipboard;
mod snarkvm;
mod dialog;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            clipboard::copy,
            dialog::warning,
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
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
