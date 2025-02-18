use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};

#[tauri::command]
pub fn warning(handle: tauri::AppHandle, _code: String) -> bool {
    let ans = handle
        .dialog()
        .message("Warning: Your changes will be lost if you don't save them!")
        .kind(MessageDialogKind::Warning)
        .title("Warning")
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Save".to_string(),
            "Don't Save".to_string(),
        ))
        .blocking_show();

    return ans;
}

#[tauri::command]
pub fn exit_warning(handle: tauri::AppHandle, _code: String) -> bool {
    let ans = handle
        .dialog()
        .message("Warning: Any unsaved changes will be lost!")
        .kind(MessageDialogKind::Warning)
        .title("Warning")
        .buttons(MessageDialogButtons::OkCancelCustom(
            "Exit".to_string(),
            "Don't Exit".to_string(),
        ))
        .blocking_show();

    return ans;
}
