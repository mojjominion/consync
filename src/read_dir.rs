use std::{fs, path::PathBuf};

use crate::{config::AppConfig, filters::is_file_interesting};

pub fn read_dir_recursive(path: PathBuf, ext: Option<&str>, app_config: &AppConfig) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    let Ok(entries) = fs::read_dir(path) else { return result };

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                let file_name = entry.file_name().to_string_lossy().to_string();

                match path.is_dir() {
                    false => {
                        if !is_file_interesting(&path, ext, app_config) {
                            continue;
                        }
                        // Process the file
                        println!("File: {:?}", path);
                        result.push(file_name);
                    }
                    true => {
                        read_dir_recursive(path, ext, app_config);
                    }
                }
            }
            Err(_) => todo!(),
        }
    }
    result
}
