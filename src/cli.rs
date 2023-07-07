use std::env::args_os;

use crate::read_dir::read_dir_recursive;

pub fn check_cli_context() -> Option<()> {
    let args: Vec<_> = args_os().collect();
    let mut files = vec![];

    match args.get(1) {
        Some(a) => {
            if a.eq("sync") {
                read_dir_recursive(dirs::config_dir().unwrap_or_default(), &mut files)
            }
            Some(())
        }
        None => None,
    }
}
