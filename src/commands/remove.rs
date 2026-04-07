//! `remove` command handler for deleting tracks from the repository.

use mcat::{
    config,
    errors::{McatError, McatResult},
    models::TrackFilter,
    repos::{Repo, toml_repo::TomlDb},
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
    if remove_file {
        todo!("crate::commands::remove::execute(): `--remove-file` not implemented yet");
    }

    let mut db: TomlDb = Repo::from(config::repo_file_path())?;
    let filter = TrackFilter::new(titles, artists, albums, album_artists, genres, hashes);

    let matched_hashes = filter.apply(&db);

    if matched_hashes.is_empty() {
        return Err(McatError::TrackNotFound);
    }

    for file_hash in matched_hashes {
        db.remove_track(&file_hash)?;
    }

    db.persist()
}
