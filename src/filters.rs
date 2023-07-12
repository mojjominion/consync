use std::{collections::HashSet, path::PathBuf};

use notify::EventKind;

use crate::read_config::AppConfig;

pub fn is_create_or_delete(kind: EventKind) -> bool {
    kind.is_create() | kind.is_remove() | kind.is_modify()
}

pub fn is_file_interesting(
    file_name: &PathBuf,
    cli_explicit_ext: Option<&str>,
    app_config: &AppConfig,
) -> bool {
    let mut exts_vec = match &app_config.file_types {
        Some(types) => {
            let config_types: Vec<&str> = types.iter().map(|x| x.as_str()).collect();
            config_types
        }
        _ => vec![],
    };

    exts_vec = if let Some(x) = cli_explicit_ext {
        vec![x]
    } else {
        exts_vec
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
