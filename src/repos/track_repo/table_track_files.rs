//! Functions associated with table "track_files".

use anyhow::{Context, Result};
use rusqlite::Transaction;

use crate::{
    common,
    models::{NewTrackFile, TrackFile},
    repos::TrackRepo,
};

pub struct TableTrackFiles;

impl TableTrackFiles {
    pub(super) fn insert_one(tx: &Transaction, track_file: NewTrackFile) -> Result<TrackFile> {
        let file_hash = common::compute_file_hash(&track_file.path)?;
        let os_str = track_file
            .path
            .file_name()
            .with_context(|| format!("Path {:?} has no file name", track_file.path))?;
        let file_name = os_str
            .to_str()
            .with_context(|| {
                format!(
                    "File name \"{}\" is not a valid UTF-8 string",
                    os_str.to_string_lossy(),
                )
            })?
            .to_string();

        let file_id = TrackRepo::insert_or_get_id(
            tx,
            "INSERT OR IGNORE INTO track_files (name, hash) VALUES (?1, ?2)",
            (&file_name, &file_hash),
            "SELECT id FROM track_files WHERE hash = ?1",
            (&file_hash,),
        )?;

        Ok(TrackFile {
            id: file_id,
            name: file_name,
            hash: file_hash,
        })
    }
}
