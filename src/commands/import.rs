//! `import` command handler for adding tracks into the repository.

use std::{fs, path::Path};

use crate::{
    config,
    errors::McatResult,
    models::TagAttributes,
    output,
    repos::{Entry, Repo, toml_repo::TomlDb},
    services::*,
};

/// Executes the `import` command.
///
/// # Errors
///
/// Returns an error if media scanning, file I/O, or repository persistence fails.
pub fn execute(path: impl AsRef<Path>, move_files: bool) -> McatResult<()> {
    let mut repo = TomlDb::try_from(config::repo_file_path())?;

    let mut entities_count = 0;
    let mut imported_count = 0;
    let mut repeated_track = Vec::new();

    let files = fs::read_dir(&path)?;
    for file in files {
        let file = file?;
        let file_path = file.path();
        let file_name = file.file_name();
        entities_count += 1;

        if file_path.is_file() && is_file_supported(&file_path)? {
            let stripped_data = strip_tags_from_file(&file_path, false)?.unwrap();
            let file_hash = get_hash_from_vec(&stripped_data);

            if let Some(entry) = repo.query_track_by_hash(&file_hash) {
                let Entry { tag_attr, .. } = entry;
                repeated_track.push(tag_attr);
            } else {
                let tag = get_primary_tag(&file_path)?;
                let mut tag_attr = TagAttributes::from(tag);
                if let Some(image) = tag_attr.front_cover {
                    tag_attr.front_cover = Some(image.linked_and_to_disk(&file_hash)?);
                }

                repo.insert_track(file_hash, tag_attr);

                if move_files {
                    fs::rename(&file_path, config::media_dir_path().join(&file_name))?;
                } else {
                    fs::copy(&file_path, config::media_dir_path().join(&file_name))?;
                }
                imported_count += 1;
            }
        }
    }

    repo.persist()?;

    println!(
        "Import result: {} file(s) / directorie(s) found, {} supported file(s) imported.",
        entities_count, imported_count,
    );
    println!("Repeated files (not imported): {}", repeated_track.len());
    output::display_as_table(&repeated_track);

    Ok(())
}
