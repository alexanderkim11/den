// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod highlight;
mod load_theme_syntax;
mod open_explorer;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            load_theme_syntax::load,
            highlight::highlight,
            open_explorer::open_explorer
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
