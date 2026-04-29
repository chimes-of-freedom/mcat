//! `display` command handler for querying and showing track metadata.

use crate::{
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
pub fn execute(filter: TrackFilter) -> McatResult<()> {
    let repo: TomlDb = Repo::from(config::repo_file_path())?;
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
