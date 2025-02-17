use tauri_plugin_clipboard_manager::ClipboardExt;

#[tauri::command]
pub async fn copy(handle: tauri::AppHandle, value: String) {
    handle.clipboard().write_text(value).unwrap();
}
