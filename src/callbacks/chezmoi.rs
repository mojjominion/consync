use std::fmt::Formatter;
use std::{fmt::Display, io::ErrorKind, ops::Not, path::Path, process::Command};

use notify::EventKind;

pub fn run_chezmoi(path: &Path, kind: EventKind) {
    if is_chezmoi().not() {
        return;
    }
    match kind {
        EventKind::Create(_) => chezmoi(path, ChezmoiArgs::Add),
        EventKind::Modify(_) => chezmoi(path, ChezmoiArgs::Add),
        EventKind::Remove(_) => chezmoi(path, ChezmoiArgs::Remove),
        _ => {}
    }
}

fn chezmoi(path: &Path, arg: ChezmoiArgs) {
    let file = path.to_string_lossy();
    println!("Executing `chezmoi {}` for {}", arg, file);

    let chezmoi = Command::new("chezmoi")
        .arg(arg.to_string())
        .arg(file.to_string())
        .arg("--force")
        .output();

    match chezmoi {
        Ok(_) => println!("Done.. {} added to chezmoi", file),
        Err(err) => println!("Errored: {}", err),
    }
}

#[allow(dead_code)]
enum ChezmoiArgs {
    Add,
    Readd,
    Remove,
}

impl Display for ChezmoiArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            ChezmoiArgs::Add => "add",
            ChezmoiArgs::Readd => "re-add",
            ChezmoiArgs::Remove => "remove",
        };
        write!(f, "{}", str)
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
