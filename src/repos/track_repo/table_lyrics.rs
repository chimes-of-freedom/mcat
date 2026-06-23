//! Functions associated with table "lyrics".

use anyhow::Result;
use rusqlite::Transaction;

use crate::{
    common,
    models::{Lyrics, NewLyrics},
    repos::TrackRepo,
};

pub struct TableLyrics;

impl TableLyrics {
    pub(super) fn insert_one(tx: &Transaction, lyrics: NewLyrics) -> Result<Lyrics> {
        let lyrics_hash = common::compute_data_hash(&lyrics.data);
        let lyrics_id = TrackRepo::insert_or_get_id(
            tx,
            "INSERT OR IGNORE INTO lyrics (hash, data) VALUES (?1, ?2)",
            (&lyrics_hash, &lyrics.data),
            "SELECT id FROM lyrics WHERE hash = ?1",
            (&lyrics_hash,),
        )?;

        Ok(Lyrics::from_new(lyrics, lyrics_id, lyrics_hash))
    }
}
