use std::fs::File;
use std::fs::exists;
use std::fs::create_dir;
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
    let file = File::create(&filepath.replace("\\","/"));
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

#[tauri::command]
pub fn mkdir(_handle: tauri::AppHandle, path: String) -> (bool, String) {
    let dir = create_dir(path);
    match dir {
        Ok(()) => (false, String::new()),
        Err(e) => (true, e.to_string())
    }
}



#[tauri::command]
pub fn read_program_json(_handle: tauri::AppHandle, filepath: String, field: String) -> String {
    let file = File::open(&filepath)
    .expect("File should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("File should be proper JSON");
    let query = json.get(&field);
    match query {
        Some(val) => return val.to_string(),
        None => return String::new()
    }
}

#[tauri::command]
pub fn path_exists(_handle: tauri::AppHandle, path: String) -> bool {
    match exists(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}