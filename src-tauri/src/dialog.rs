use rfd::{MessageButtons, MessageDialog};

#[tauri::command]
pub fn warning(_handle: tauri::AppHandle, _code: String) -> String {
    let warning = MessageDialog::new()
        .set_title("Warning")
        .set_level(rfd::MessageLevel::Warning)
        .set_description("Warning: Your changes will be lost if you don't save them!")
        .set_buttons(MessageButtons::YesNoCancelCustom("Save".to_string(), "Don't Save".to_string(), "Cancel".to_string()))
        .show();
    
    match warning {
        rfd::MessageDialogResult::Custom(val) => {
            return val;
        }
        _ => return "Cancel".to_string()
    }

}

#[tauri::command]
pub fn exit_warning(_handle: tauri::AppHandle, _code: String) -> bool {
    let warning = MessageDialog::new()
        .set_title("Warning")
        .set_level(rfd::MessageLevel::Warning)
        .set_description("Warning: Any unsaved changes will be lost!")
        .set_buttons(MessageButtons::OkCancelCustom("Exit".to_string(), "Don't Exit".to_string()))
        .show();
    
    match warning {
        rfd::MessageDialogResult::Custom(val) => {
            if val == "Exit".to_string() {
                return true
            } else {
                return false
            }
        }
        _ => return false
    }

}

