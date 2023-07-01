use std::{ffi::OsString, path::PathBuf};

use notify::EventKind;

pub fn is_create_or_delete(kind: EventKind) -> bool {
    kind.is_create() | kind.is_remove() | kind.is_modify()
}
pub fn is_file_interesting(file_name: &PathBuf) -> bool {
    let interesting_exts = [
        OsString::from("conf"),
        OsString::from("confg"),
        OsString::from("config"),
    ];
    let ext = &file_name
        .extension()
        .unwrap_or_default()
        .to_ascii_lowercase();

    interesting_exts.contains(ext)
}
