use std::{io::ErrorKind, ops::Not, path::PathBuf, process::Command};

use notify::EventKind;

pub fn run_chezmoi(path: &PathBuf, kind: EventKind) {
    if is_chezmoi().not() {
        return;
    }
    match kind {
        EventKind::Create(_) => chezmoi(path, ChezmoiArgs::ADD),
        EventKind::Modify(_) => chezmoi(path, ChezmoiArgs::ADD),
        EventKind::Remove(_) => chezmoi(path, ChezmoiArgs::REMOVE),
        _ => {}
    }
}

fn chezmoi(path: &PathBuf, arg: ChezmoiArgs) {
    let file = path.to_string_lossy();
    println!("Executing `chezmoi {}` for {}", arg.to_string(), file);

    let chezmoi = Command::new("chezmoi")
        .arg(arg.to_string())
        .arg(file.to_string())
        .arg("--force")
        .output();

    match chezmoi {
        Ok(_) => println!("Done.."),
        Err(err) => println!("Errored: {}", err),
    }
}

#[allow(dead_code)]
enum ChezmoiArgs {
    ADD,
    READD,
    REMOVE,
}
impl ToString for ChezmoiArgs {
    fn to_string(&self) -> String {
        let str = match self {
            ChezmoiArgs::ADD => "add",
            ChezmoiArgs::READD => "re-add",
            ChezmoiArgs::REMOVE => "remove",
        };
        str.to_string()
    }
}

pub fn is_chezmoi() -> bool {
    match Command::new("chezmoi").output() {
        Ok(_) => true,
        Err(e) => {
            match e.kind() {
                ErrorKind::NotFound => {
                    println!("`chezmoi` was not found! Check your PATH!")
                }
                _ => {
                    println!("Some strange error occurred :(");
                }
            }
            false
        }
    }
}
