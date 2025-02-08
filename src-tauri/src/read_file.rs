use std::fs::File;
use std::io::prelude::*;


#[tauri::command]
pub fn read_file(_handle: tauri::AppHandle, filepath: String) -> Option<String> {
    let mut contents = String::new();
    let file = File::open(&filepath);
    match file {
        Ok(mut f) => {
            f.read_to_string(&mut contents).unwrap();
            Some(contents)
        },
        Err(_e) => {
            None
        }
    }
}
