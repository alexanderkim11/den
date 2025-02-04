use std::fs::File;
use std::io::prelude::*;


#[tauri::command]
pub fn read_file(_handle: tauri::AppHandle, filepath: String) -> String {
    let mut file = File::open(&filepath).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    return contents;
}
