//! `check` command handler for validating repository and media consistency.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    errors::McatResult,
    models::CheckResult,
    repos::{Repository, toml_repo::Database},
    services::{get_hash_from_vec, is_file_supported, strip_tags_from_file},
};

pub fn execute(
    track: bool,
    exist: bool,
    repair: bool,
    save_to: Option<impl AsRef<Path>>,
) -> McatResult<()> {
    if repair {
        todo!("Arg `--repair` is not implemented yet");
    }

    let db: Database = Repository::from(PathBuf::from(".mcat/db.toml"))?;
    let db_keys = db.get_track_hashes();

    let mut file_hashes = BTreeSet::new();

    let files = fs::read_dir("media/")?;

    for file in files {
        let file = file?;
        let file_type = file.file_type()?;
        let file_path = file.path();

        if file_type.is_file() && is_file_supported(&file_path)? {
            let stripped_data = strip_tags_from_file(&file_path, false)?.unwrap();
            let file_hash = get_hash_from_vec(&stripped_data);
            file_hashes.insert(file_hash);
        }
    }

    let not_tracked = if !exist {
        Some(&file_hashes - &db_keys)
    } else {
        None
    };
    let not_exists = if !track {
        Some(&db_keys - &file_hashes)
    } else {
        None
    };

    // save the result to `save_path`
    if let Some(save_path) = save_to {
        let check_res = CheckResult {
            not_tracked,
            not_exists,
        };
        let check_res = toml::to_string(&check_res)?;
        fs::write(&save_path, &check_res)?;
    }

    Ok(())
}
