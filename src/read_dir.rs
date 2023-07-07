use std::{fs, path::PathBuf};

use crate::filters::is_file_interesting;

pub fn read_dir_recursive(path: PathBuf, result: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(path) else { return };

    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            let file_name = entry.file_name().to_string_lossy().to_string();

            match path.is_dir() {
                false => {
                    if !is_file_interesting(&path) {
                        continue;
                    }
                    // Process the file
                    println!("File: {:?}", path);
                    result.push(file_name);
                }
                true => {
                    read_dir_recursive(path, result);
                }
            }
        }
    }
}
