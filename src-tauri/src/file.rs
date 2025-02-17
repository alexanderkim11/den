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
        }
        Err(_e) => None,
    }
}

#[tauri::command]
pub fn write_file(_handle: tauri::AppHandle, filepath: String, contents: String) -> (bool, String) {
    let file = File::options().read(true).write(true).open(&filepath);
    match file {
        Ok(mut f) => {
            let call = f.write_all(contents.as_bytes());
            match call {
                Ok(()) => (false, String::new()),
                Err(e) => (true, e.to_string()),
            }
        }
        Err(e) => (true, e.to_string()),
    }
}
