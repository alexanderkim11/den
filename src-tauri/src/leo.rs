use std::env::current_dir;
use tauri_plugin_shell::ShellExt;


#[tauri::command]
pub async fn execute(handle: tauri::AppHandle, command: Vec<String>) -> (bool, String) {
    let current_dir = current_dir().unwrap();
    let exec_bin = format!("{}{}", current_dir.as_path().to_str().unwrap(), "/.leo/leo.exe");
    let shell = handle.shell();
    let output = shell
            .command(exec_bin)
            .args(command)
            .output()
            .await
            .unwrap();
    if output.status.success() {
        return (true, String::from_utf8(output.stdout).unwrap())
    } else {
        return (false, String::from_utf8(output.stderr).unwrap())
    }
    
}




