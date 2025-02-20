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
pub fn read_program_json(_handle: tauri::AppHandle, filepath: String) -> String {
    println!("{}",filepath);
    let file = File::open(&filepath)
    .expect("File should open read only");
    let json: serde_json::Value = serde_json::from_reader(file)
        .expect("File should be proper JSON");
    let first_name = json.get("program")
        .expect("File should have program name");
    return first_name.to_string()
}