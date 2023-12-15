use crate::{config::AppConfig, read_dir::read_dir_recursive};

use super::{args::get_arg, usage::print_usage};

pub fn cli_context(app_config: &AppConfig) -> Option<()> {
    match get_arg(1) {
        Some(a) => {
            match a.as_str() {
                "find" => {
                    let ext = get_arg(2);
                    let files = read_dir_recursive(
                        dirs::config_dir().unwrap_or_default(),
                        ext.as_deref(),
                        app_config,
                    );
                    for pat in files {
                        println!("{}", pat)
                    }
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
