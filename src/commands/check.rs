//! `check` command handler for validating repository and media consistency.

use std::{collections::BTreeSet, fs, path::Path};

use mcat::{
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
    let mut db: TomlDb = Repo::from(config::repo_file_path())?;
    let db_keys = db.get_track_hashes();

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

    // `None` implies a filter is applied
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

    // apply the result to the repo
    if repair {
        // insert untracked tracks into repo
        if !exist {
            for (file_path, file_hash) in files_not_tracked {
                let tag = get_primary_tag(&file_path)?;
                let mut tag_attr = TagAttributes::from_tag(&tag);
                if let Some(image) = tag_attr.front_cover {
                    tag_attr.front_cover = Some(image.linked_and_to_disk(&file_hash)?);
                }
                db.insert_track(file_hash, tag_attr);
            }
        }

        // delete tracks not existing under `media/`
        if !track {
            for file_hash in not_exists.as_ref().unwrap() {
                db.remove_track(file_hash)?;
            }
        }

        db.persist()?;
    }

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
