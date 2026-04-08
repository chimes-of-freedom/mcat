//! `display` command handler for querying and showing track metadata.

use mcat::{
    config,
    errors::McatResult,
    models::{TagAttributes, TrackFilter},
    output::display_as_table,
    repos::{Repo, toml_repo::TomlDb},
};

/// Executes the `display` command.
///
/// # Errors
///
/// Returns repository loading errors.
pub fn execute(
    titles: Vec<String>,
    artists: Vec<String>,
    albums: Vec<String>,
    album_artists: Vec<String>,
    genres: Vec<String>,
    hashes: Vec<String>,
) -> McatResult<()> {
    let repo: TomlDb = Repo::from(config::repo_file_path())?;
    let filter = TrackFilter::new(titles, artists, albums, album_artists, genres, hashes);
    let track_hashes = filter.apply(&repo);

    let mut tag_attrs = Vec::new();

    for track_hash in track_hashes {
        if let Some(entry) = repo.query_track_by_hash(&track_hash) {
            tag_attrs.push(entry.tag_attr);
        }
    }

    let tag_attrs: Vec<&TagAttributes> = tag_attrs.iter().collect();
    display_as_table(&tag_attrs);

    Ok(())
}
