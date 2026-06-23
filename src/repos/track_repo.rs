//! The repository for tracks.

mod table_images;
mod table_lyrics;
mod table_track_files;
mod table_tracks;

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use rusqlite::{Connection, Params, Transaction, types::Value};

use crate::{
    models::{NewTrack, Patch, Track, TrackFilter, TrackRow},
    repos::track_repo::table_tracks::TableTracks,
};

pub struct TrackRepo;

impl TrackRepo {
    pub fn init(tx: &Transaction) -> Result<()> {
        tx.execute_batch(
            "
                CREATE TABLE lyrics (
                    id    INTEGER PRIMARY KEY,
                    hash  BLOB NOT NULL UNIQUE,
                    data  BLOB
                );

                CREATE TABLE images (
                    id    INTEGER PRIMARY KEY,
                    hash  BLOB NOT NULL UNIQUE,
                    data  BLOB
                );

                CREATE TABLE track_files (
                    id    INTEGER PRIMARY KEY,
                    name  TEXT NOT NULL,
                    hash  BLOB NOT NULL UNIQUE
                );

                CREATE TABLE tracks (
                    id              INTEGER PRIMARY KEY,
                    title           TEXT NOT NULL,
                    artist          TEXT,
                    album           TEXT,
                    album_artist    TEXT,
                    recording_date  TEXT,
                    release_date    TEXT,
                    track_number    INTEGER,
                    disc_number     INTEGER,
                    genre           TEXT,
                    composer        TEXT,
                    lyricist        TEXT,
                    lyrics_id       INTEGER REFERENCES lyrics(id),
                    front_cover_id  INTEGER REFERENCES images(id),
                    file_id         INTEGER REFERENCES track_files(id)
                );

                PRAGMA foreign_keys = ON;
                ",
        )
        .context("rusqlite init track tables failed")?;

        Ok(())
    }

    /// Inserts a track into the repository and returns a [`Track`].
    ///
    /// - BLOB fields are inserted into their own tables.
    /// - The track file is copied to `media/` if outside,
    ///   and a [`TrackFile`] is inserted into table "track_files".
    ///
    /// # Errors
    ///
    /// Returns an error in the following situations,
    /// but is not limited to just these cases:
    ///
    /// - The file path provided by `new_track` terminates in `..`.
    /// - Fails to copy the track file to `media/`.
    /// - Fails to read the track file when computing it's hash.
    /// - SQL operations go wrong.
    pub fn insert_track(tx: &Transaction, new_track: NewTrack) -> Result<Track> {
        TableTracks::insert_one(tx, new_track)
    }

    /// Inserts a vec of tracks into the repository and returns [`Vec<Track>`].
    /// 
    /// See also [`Self::insert_track`].
    pub fn insert_tracks(tx: &Transaction, new_tracks: Vec<NewTrack>) -> Result<Vec<Track>> {
        TableTracks::insert_many(tx, new_tracks)
    }

    pub fn select_tracks_by_filter(
        conn: &Connection,
        filter: &TrackFilter,
    ) -> Result<Vec<TrackRow>> {
        TableTracks::select_by_filter(conn, filter)
    }

    pub fn remove_tracks_by_filter(
        tx: &Transaction,
        filter: &TrackFilter,
    ) -> Result<Vec<TrackRow>> {
        TableTracks::remove_by_filter(tx, filter)
    }

    pub fn remove_tracks_by_id(tx: Transaction, table_name: &str, ids: &[i64]) -> Result<i64> {
        TableTracks::remove_by_id(tx, table_name, ids)
    }

    pub fn update_tracks_by_filter(
        tx: &Transaction,
        filter: &TrackFilter,
        cols_patched: &mut HashMap<String, Patch>,
    ) -> Result<Vec<TrackRow>> {
        TableTracks::update_by_filter(tx, filter, cols_patched)
    }

    fn insert_or_get_id(
        tx: &Transaction,
        insert_sql: &str,
        insert_params: impl Params,
        select_sql: &str,
        select_params: impl Params,
    ) -> Result<i64> {
        let rows_changed = tx.execute(insert_sql, insert_params)?;
        if rows_changed != 0 {
            Ok(tx.last_insert_rowid())
        } else {
            Ok(tx.query_one(select_sql, select_params, |row| row.get("id"))?)
        }
    }

    /// Builds where template like `WHERE artist = ? AND track_number = ? OR
    /// disc_number = ?`, returning the template along with it's parameters.
    /// Empty string and vec are returned if the filter is empty.
    fn build_where_clause(filter: &TrackFilter) -> (String, Vec<Value>) {
        let mut conditions = Vec::new();
        let mut params = Vec::new();

        Self::add_i64_filter(&mut conditions, &mut params, "id", &filter.ids);
        Self::add_string_filter(&mut conditions, &mut params, "title", &filter.titles);
        Self::add_string_filter(&mut conditions, &mut params, "artist", &filter.artists);
        Self::add_string_filter(&mut conditions, &mut params, "album", &filter.albums);
        Self::add_string_filter(
            &mut conditions,
            &mut params,
            "album_artist",
            &filter.album_artists,
        );
        Self::add_string_filter(
            &mut conditions,
            &mut params,
            "recording_date",
            &filter.recording_dates,
        );
        Self::add_string_filter(
            &mut conditions,
            &mut params,
            "release_date",
            &filter.release_dates,
        );
        Self::add_i64_filter(
            &mut conditions,
            &mut params,
            "track_number",
            &filter.track_numbers,
        );
        Self::add_i64_filter(
            &mut conditions,
            &mut params,
            "disc_number",
            &filter.disc_numbers,
        );
        Self::add_string_filter(&mut conditions, &mut params, "genre", &filter.genres);
        Self::add_string_filter(&mut conditions, &mut params, "composer", &filter.composers);
        Self::add_string_filter(&mut conditions, &mut params, "lyricist", &filter.lyricists);

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        (where_clause, params)
    }

    /// Builds set template like `SET artist = ?, track_number = ?`, returning
    /// the template along with it's parameters.
    /// Empty string and vec are returned if the filter is empty.
    ///
    /// # Errors
    ///
    /// Returns an error if failing to parse a string value to [`i64`] as
    /// expected.
    fn build_set_clause(cols_patched: &HashMap<String, Patch>) -> Result<(String, Vec<Value>)> {
        let mut conditions = Vec::new();
        let mut params = Vec::new();
        let cols_i64 = HashSet::from(["track_number", "disc_number"]);

        let string2val = |col: &str, val: &String| -> Result<Value> {
            if cols_i64.contains(col) {
                let val = val
                    .parse()
                    .with_context(|| format!("Value {val} cannot be parsed to an i64"))?;
                Ok(Value::Integer(val))
            } else {
                Ok(Value::Text(val.clone()))
            }
        };

        for (col, patch) in cols_patched {
            match patch {
                Patch::Set(val) => {
                    let val = string2val(col, val)?;
                    Self::add_single_value(&mut conditions, &mut params, col, val)
                }
                Patch::Keep => {}
                Patch::Clear => {
                    Self::add_single_value(&mut conditions, &mut params, col, Value::Null)
                }
            }
        }

        let set_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("SET {}", conditions.join(", "))
        };

        Ok((set_clause, params))
    }

    /// Adds `String` filter to conditions and updates params.
    fn add_string_filter(
        conditions: &mut Vec<String>,
        params: &mut Vec<Value>,
        col: &str,
        values: &HashSet<String>,
    ) {
        if values.is_empty() {
            return;
        }

        let mut parts = vec![];
        for value in values {
            Self::add_single_value(&mut parts, params, col, Value::Text(value.clone()));
        }

        conditions.push(parts.join(" OR "));
    }

    /// Adds `i64` filter to conditions and updates params.
    fn add_i64_filter(
        conditions: &mut Vec<String>,
        params: &mut Vec<Value>,
        col: &str,
        values: &HashSet<i64>,
    ) {
        if values.is_empty() {
            return;
        }

        let mut parts = vec![];
        for value in values {
            Self::add_single_value(&mut parts, params, col, Value::Integer(*value));
        }

        conditions.push(parts.join(" OR "));
    }

    /// Adds a single filter to conditions and updates params.
    fn add_single_value(
        conditions: &mut Vec<String>,
        params: &mut Vec<Value>,
        col: &str,
        value: Value,
    ) {
        conditions.push(format!("{col} = ?"));
        params.push(value);
    }

    fn check_table_name(table_name: &str) -> bool {
        ["tracks", "lyrics", "images", "track_files"].contains(&table_name)
    }
}
