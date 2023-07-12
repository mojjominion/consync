use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct FileType {
    /// ie. yml, toml, service, conf etc
    pub file_type: String,
    /// This command will be executed for the file type set `file_type`
    ///
    ///
    /// For Example:
    /// `run: "cat"`
    /// This cammand witl just print the content of the file having `file_type` extention
    ///
    /// **Note:** If this is set global `run` will be ignored for the file type
    pub run: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub root: String,

    /// This command will be executed for the file type set global `file_type`
    ///
    ///
    /// For Example:
    /// `run: "cat"`
    /// This cammand witl just print the content of the file having `file_type` extention
    ///
    pub run: Option<String>,
    pub file_types: Option<Vec<String>>,
}

fn read_config_file(path: &Path) -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let config = serde_yaml::from_str(&contents)?;
    Ok(config)
}

fn write_config_file(path: &Path, config: &AppConfig) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = OpenOptions::new().write(true).create(true).open(path)?;
    let json_config = serde_yaml::to_string(config)?;
    file.write_all(json_config.as_bytes())?;
    file.sync_all()?;
    Ok(())
}

pub fn load_config() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let mut config_path = dirs::config_dir().unwrap_or_default();
    config_path.push("consync");
    config_path.push("consync.yml");

    let root = dirs::config_dir()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let config = match read_config_file(config_path.as_path()) {
        Ok(config) => {
            println!("Existing configuration: {:#?}", config);
            config
        }
        Err(_) => {
            let types = vec!["conf", "config", "confg", "yml", "yaml", "service"]
                .iter()
                .map(|x| x.to_string())
                .collect();
            let new_config = AppConfig {
                root,
                run: Some("cat".to_string()),
                file_types: Some(types),
            };
            let res = write_config_file(config_path.as_path(), &new_config)?;
            println!("Created new configuration: {:#?}", new_config);
            println!("{res:#?}");
            new_config
        }
    };

    // Use the config for further processing
    Ok(config)
}
