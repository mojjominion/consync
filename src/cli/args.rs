use std::env::args_os;

pub fn get_arg(index: usize) -> Option<String> {
    args_os()
        .nth(index)
        .map(|arg| arg.to_string_lossy().into_owned())
}
