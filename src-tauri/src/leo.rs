use std::env::current_dir;
use tauri_plugin_shell::ShellExt;
use tauri_plugin_os::*;

#[tauri::command]
pub async fn execute(handle: tauri::AppHandle, mut command: Vec<String>) -> (bool, String) {
    let current_dir = current_dir().unwrap();
    let platform = platform();
    let bin = if platform == "windows" {"leo-x86_64-pc-windows-msvc.exe"} else if platform == "macos" { if arch() == "aarch64" {"leo-aarch64-apple-darwin"} else {"leo-x86_64-apple-darwin"} } else {"x86_64-unknown-linux-gnu"};
    let exec_bin = format!(
        "{}{}{}",
        current_dir.as_path().to_str().unwrap(),
        "/.leo/",
        bin
    );
    command.push("--home".to_string());
    command.push(format!("{}{}",current_dir.as_path().to_str().unwrap(),"/.aleo/"));
    let shell = handle.shell();
    let output = shell
        .command(exec_bin)
        .args(command)
        .output()
        .await
        .unwrap();
    if output.status.success() {
        return (false, String::from_utf8(output.stdout).unwrap());
    } else {
        return (true, String::from_utf8(output.stderr).unwrap());
    }
}
