use std::{path::PathBuf, process::Command};

use notify::EventKind;

use crate::config::AppConfig;

use super::chezmoi;

#[derive(Debug)]
pub struct Executor {
    pub app_config: AppConfig,
    pub file_name: String,
    pub kind: EventKind,
}

impl Executor {
    pub fn run(self) {
        match self.app_config.run {
            Some(global_cmd) => {
                let output = Command::new(global_cmd.to_string())
                    .arg(self.file_name.to_string())
                    .output();

                match output {
                    Ok(res) => println!("{:#?}", String::from_utf8(res.stdout)),
                    Err(err) => println!("Errored: {:#?}", err),
                }
            }
            _ => chezmoi::run_chezmoi(&PathBuf::from(self.file_name), self.kind),
        }
    }
}
