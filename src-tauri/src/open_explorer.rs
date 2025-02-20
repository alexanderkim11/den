use tauri_plugin_dialog::DialogExt;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct CustomDirEntry {
    path: String,
    name: String,
    type_of: String,
    subpaths: Vec<CustomDirEntry>,
}

pub fn recurse_walk_dir(folder_path: &Path) -> Vec<CustomDirEntry> {
    let mut return_vec: Vec<CustomDirEntry> = Vec::new();
    let paths = fs::read_dir(folder_path).unwrap();
    for entry in paths {
        let dir_entry = entry.unwrap();
        let path = dir_entry.path();
        let entry_name = (&path).file_name().unwrap().to_str().unwrap().to_string();
        let path_string = path.to_str().unwrap().to_string();
        let metadata = dir_entry.metadata();
        let is_dir = metadata.unwrap().is_dir();
        let entry_type_string = if is_dir {
            "Directory".to_string()
        } else {
            "File".to_string()
        };
        let subpaths_vec;
        if is_dir {
            subpaths_vec = recurse_walk_dir(&path);
        } else {
            subpaths_vec = Vec::new();
        }
        return_vec.push(CustomDirEntry {
            name: entry_name,
            path: path_string,
            type_of: entry_type_string,
            subpaths: subpaths_vec,
        });
    }

    return return_vec;
}

#[tauri::command]
pub fn open_explorer(handle: tauri::AppHandle, _code: String) -> String {
    let default_path = Path::new("C:\\Users\\r0ami\\Home\\aleo\\projects\\den-test\\test2");
    let folder_path_option = handle
        .dialog()
        .file()
        .set_directory(default_path)
        .blocking_pick_folder();

    let mut return_val: String = String::new();
    match folder_path_option {
        Some(folder_path) => {
            let folder_path = folder_path.as_path().unwrap();
            let this_folder = recurse_walk_dir(&folder_path);

            let mut full_return_vec: Vec<CustomDirEntry> = Vec::new();
            full_return_vec.push(CustomDirEntry {
                name: folder_path.file_name().unwrap().to_str().unwrap().to_string(),
                path: folder_path.to_str().unwrap().to_string(),
                type_of: "Directory".to_string(),
                subpaths: this_folder,
            });
            return_val = serde_json::to_string(&full_return_vec).unwrap();
        }
        None => {
            return_val = "".to_string();
        }
    }
    return return_val;
}


#[tauri::command]
pub fn get_directory(_handle: tauri::AppHandle, directory: String) -> String {
    let folder_path = Path::new(&directory);
    let this_folder = recurse_walk_dir(&folder_path);

    let mut full_return_vec: Vec<CustomDirEntry> = Vec::new();
    full_return_vec.push(CustomDirEntry {
        name: folder_path.file_name().unwrap().to_str().unwrap().to_string(),
        path: folder_path.to_str().unwrap().to_string(),
        type_of: "Directory".to_string(),
        subpaths: this_folder,
    });
    let return_val = serde_json::to_string(&full_return_vec).unwrap();

    return return_val;
}