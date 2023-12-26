use std::{collections::HashSet, path::PathBuf, vec};

use notify::EventKind;

use crate::config::AppConfig;

pub fn is_create_or_delete(kind: EventKind) -> bool {
    kind.is_create() | kind.is_remove() | kind.is_modify()
}

fn wild_prefix(str: &str) -> Option<&str> {
    let wild_chars = [".", "*"];
    wild_chars.into_iter().find(|x| str.starts_with(x))
}

pub fn is_file_interesting(
    file_path: &PathBuf,
    cli_explicit_ext: Option<&str>,
    app_config: &AppConfig,
) -> bool {
    let file_name = file_path
        .file_name()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default();

    let file_ext = file_path
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
        .to_owned();

    let mut exts_vec = match &app_config.file_types {
        Some(types) => {
            let config_types: Vec<&str> = types.iter().map(|x| x.as_str()).collect();
            config_types
        }
        _ => vec![],
    };
    exts_vec = match cli_explicit_ext {
        Some(x) if wild_prefix(x).is_some() => {
            if let Some(stripped) = x.strip_prefix(wild_prefix(x).unwrap_or_default()) {
                vec![stripped]
            } else {
                exts_vec
            }
        }
        Some(x) => vec![x],
        None => exts_vec,
    };

    let file_exts_set: HashSet<&str> = exts_vec.iter().cloned().collect();

    let is_with_ext = file_exts_set.contains(file_ext.as_str());
    let is_suffix = exts_vec.iter().any(|x| file_name.ends_with(x));

    is_with_ext || is_suffix
}
