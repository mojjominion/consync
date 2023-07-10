use std::env::args_os;

use crate::read_dir::read_dir_recursive;

pub fn get_arg(index: usize) -> Option<String> {
    args_os()
        .skip(index)
        .next()
        .map(|arg| arg.to_string_lossy().into_owned())
}

fn print_usage(program_name: &str) {
    println!("Usage: {} [options] <input>", program_name);
    println!();
    println!("Options:");
    println!("    help     Print this help message");
    println!("    find     Print the version information");
    // Add more options here
    println!("Input:");
    println!("    <input>  File extension, e.g., config, yaml");
}

pub fn check_cli_context() -> Option<()> {
    let mut files = vec![];

    match get_arg(1) {
        Some(a) => {
            match a.as_str() {
                "find" => {
                    let ext = get_arg(2);
                    read_dir_recursive(
                        dirs::config_dir().unwrap_or_default(),
                        &mut files,
                        ext.as_deref(),
                    )
                }
                "help" => print_usage("consync"),
                other_value => {
                    println!("{other_value:#?} Command not found")
                }
            }
            Some(())
        }
        _ => None,
    }
}
