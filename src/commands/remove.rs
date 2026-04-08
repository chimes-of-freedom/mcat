//! `remove` command handler for deleting tracks from the repository.

use std::fs;

use mcat::{
    config,
    errors::{McatError, McatResult},
    models::TrackFilter,
    repos::{Repo, toml_repo::TomlDb},
    services::*,
};

/// Executes the `remove` command.
///
/// # Errors
///
/// Returns repository loading, lookup, removal, or persistence errors.
pub fn execute(
    titles: Vec<String>,
    artists: Vec<String>,
    albums: Vec<String>,
    album_artists: Vec<String>,
    genres: Vec<String>,
    hashes: Vec<String>,
    remove_file: bool,
) -> McatResult<()> {
    let mut db: TomlDb = Repo::from(config::repo_file_path())?;
    let filter = TrackFilter::new(titles, artists, albums, album_artists, genres, hashes);

    let matched_hashes = filter.apply(&db);

    if matched_hashes.is_empty() {
        return Err(McatError::TrackNotFound);
    }

    for file_hash in &matched_hashes {
        db.remove_track(file_hash)?;
    }

    if remove_file {
        let files = fs::read_dir(config::media_dir_path())?;

        for file in files {
            let file = file?;
            let file_path = file.path();
            let file_name = file.file_name();
            let file_type = file.file_type()?;

            if file_type.is_file() && is_file_supported(&file_path)? {
                let stripped_data = strip_tags_from_file(&file_path, false)?.unwrap();
                let file_hash = get_hash_from_vec(&stripped_data);
                if matched_hashes.contains(&file_hash) {
                    fs::remove_file(&file_path)?;
                    println!(
                        "Removed file \"{}\"",
                        file_name
                            .to_str()
                            .unwrap_or("[WARNING] not a valid UTF-8 file name"),
                    );
                }
            }
        }
    }

    db.persist()
}
