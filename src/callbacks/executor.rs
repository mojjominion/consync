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
                    Ok(res) => {
                        // println!("{:?}", self.kind);
                        pretty_print_output(&res.stdout);
                    }
                    Err(err) => pretty_print_output(err.to_string().as_bytes()),
                }
            }
            _ => chezmoi::run_chezmoi(&PathBuf::from(self.file_name), self.kind),
        }
    }
}

fn pretty_print_output(bytes: &[u8]) {
    let output_str = String::from_utf8_lossy(bytes);

    for line in output_str.lines() {
        if line.contains("error") {
            // Print error messages in red
            println!("\x1b[91m{}\x1b[0m", line);
        } else if line.contains("warning") {
            // Print warning messages in yellow
            println!("\x1b[93m{}\x1b[0m", line);
        } else {
            // Print other lines in the default color
            println!("{}", line);
        }
    }
}
