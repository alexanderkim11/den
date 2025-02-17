
use tauri_plugin_opener::OpenerExt;


#[tauri::command]
pub fn open_url(handle: tauri::AppHandle, url : String) {
    handle.opener().open_url(url, None::<&str>).expect("Error with opening URL");
}