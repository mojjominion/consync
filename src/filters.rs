use std::{collections::HashSet, path::PathBuf};

use notify::EventKind;

pub fn is_create_or_delete(kind: EventKind) -> bool {
    kind.is_create() | kind.is_remove() | kind.is_modify()
}
pub fn is_file_interesting(file_name: &PathBuf, ext: Option<&str>) -> bool {
    let exts_vec = if let Some(x) = ext {
        vec![x]
    } else {
        vec!["conf", "config", "confg", "yml", "yaml", "service"]
    };

    let file_exts_set: HashSet<&str> = exts_vec.iter().cloned().collect();
    let file_begins = vec!["."];

    let file_ext = file_name
        .extension()
        .unwrap_or_default()
        .to_str()
        .map(|ext| ext.to_lowercase())
        .unwrap_or_default();

    let ext_matched = file_exts_set.contains(file_ext.as_str());
    let begining_matched = file_begins.iter().any(|x| file_name.starts_with(x));

    ext_matched || begining_matched
}
