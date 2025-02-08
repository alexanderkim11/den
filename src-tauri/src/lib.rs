// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod highlight;
mod load_theme_syntax;
mod open_explorer;
mod read_file;
mod compile;
// mod watch;
// use tauri::Listener;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct DownloadStarted<'a> {
    url: &'a str,
    download_id: usize,
    content_length: usize,
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        // .setup(|app| {
        //     app.listen("download-started", |event| {
        //       if let Ok(payload) = serde_json::from_str::<DownloadStarted>(&event.payload()) {
        //         println!("downloading {}", payload.url);
        //       }
        //     });
        //     Ok(())
        //   })
        .invoke_handler(tauri::generate_handler![
            load_theme_syntax::load,
            highlight::highlight,
            open_explorer::open_explorer,
            read_file::read_file,
            compile::compile,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
