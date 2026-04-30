//! `check` command handler for validating repository and media consistency.

use std::{collections::BTreeSet, fs, path::Path};

use crate::{
    config,
    errors::McatResult,
    models::{CheckResult, TagAttributes},
    repos::{Repo, toml_repo::TomlDb},
    services::*,
};

/// Executes the `check` command.
///
/// # Errors
///
/// Returns file-system, repository, metadata, serialization, or hashing errors
/// encountered during scanning, optional repair, or optional result export.
pub fn execute(
    track: bool,
    exist: bool,
    repair: bool,
    save_to: Option<impl AsRef<Path>>,
) -> McatResult<()> {
    let mut repo = TomlDb::try_from(config::repo_file_path())?;
    let track_hashes = repo.get_track_hashes();

    let mut file_hashes = BTreeSet::new();

    let mut files_not_tracked = Vec::new();

    let files = fs::read_dir(config::media_dir_path())?;

    for file in files {
        let file = file?;
        let file_type = file.file_type()?;
        let file_path = file.path();

        if file_type.is_file() && is_file_supported(&file_path)? {
            let stripped_data = strip_tags_from_file(&file_path, false)?.unwrap();
            let file_hash = get_hash_from_vec(&stripped_data);
            file_hashes.insert(file_hash.clone());
            files_not_tracked.push((file_path, file_hash));
        }
    }

    let not_tracked = if !exist {
        &file_hashes - &track_hashes
    } else {
        BTreeSet::new()
    };
    let not_exists = if !track {
        &track_hashes - &file_hashes
    } else {
        BTreeSet::new()
    };

    // apply the result to the repo
    if repair {
        // insert untracked tracks into repo
        if !exist {
            for (file_path, file_hash) in files_not_tracked {
                let tag = get_primary_tag(&file_path)?;
                let tag_attr = TagAttributes::from(tag);
                repo.insert_track(file_hash, tag_attr)?;
            }
        }

        // delete tracks not existing under `media/`
        if !track {
            for file_hash in &not_exists {
                repo.remove_track(file_hash)?;
            }
        }

        repo.persist()?;
    }

    // save the result to `save_path` or print a message on terminal
    if let Some(save_path) = save_to {
        let check_res = CheckResult {
            not_tracked,
            not_exists,
        };
        let check_res = toml::to_string(&check_res)?;
        fs::write(&save_path, &check_res)?;
    } else {
        println!(
            "Check result: {:?} not tracked, {:?} not exist(s).",
            not_tracked.len(),
            not_exists.len()
        );
    }

    Ok(())
}
