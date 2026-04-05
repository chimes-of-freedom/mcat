//! `remove` command handler for deleting tracks from the repository.

use mcat::{
    errors::McatResult,
    repos::{Entry, Repo, toml_repo::TomlDb},
    services::is_valid_blake3_hex,
};

pub fn execute(track: &str, remove_file: bool) -> McatResult<()> {
    if remove_file {
        todo!("crate::commands::remove::execute(): `--remove-file` not implemented yet");
    }

    let mut db: TomlDb = Repo::from(".mcat/db.toml")?;

    let entry = if is_valid_blake3_hex(track) {
        db.query_track_by_hash(track)
    } else {
        db.query_track_by_title(&track)
    };

    let Some(Entry { file_hash, .. }) = entry else {
        panic!();
    };

    db.remove_track(&file_hash)?;

    db.persist()
}
