//! The repository for tracks.

use std::{
    collections::{HashMap, HashSet},
    fs,
    path::Path,
};

use anyhow::{Context, Result, ensure};
use rusqlite::{Connection, Params, params_from_iter, types::Value};

use crate::{
    common,
    models::{Image, Lyrics, NewTrack, Patch, Track, TrackFile, TrackFilter, TrackRow},
};

pub struct TrackRepo<'a> {
    conn: &'a Connection,
}

impl<'a> TrackRepo<'a> {
    pub fn new(conn: &'a Connection) -> Self {
        Self { conn }
    }

    pub fn init(&self) -> Result<()> {
        self.conn
            .execute_batch(
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

                CREATE TABLE files (
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
                    file_id         INTEGER REFERENCES files(id)
                );
                ",
            )
            .context("rusqlite init track tables failed")?;

        Ok(())
    }

    /// Inserts a track into [`TrackRepo`] and returns a [`Track`].
    ///
    /// - BLOB fields are inserted into their own tables.
    /// - The track file is copied to `media/` if outside,
    ///   and a [`TrackFile`] is inserted into table "files".
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
    pub fn insert(&mut self, new_track: NewTrack) -> Result<Track> {
        let file_name = new_track.file.path.file_name().unwrap();
        let file_path = Path::new("media").join(file_name);

        // Copy file to "media/".
        common::atomic_copy(&new_track.file.path, &file_path)?;

        // Insert `TrackFile` into table "files".
        let file_hash = common::compute_file_hash(&file_path)?;
        let file_id = self.insert_or_get_id(
            "INSERT OR IGNORE INTO files (name, hash) VALUES (?1, ?2)",
            (file_name.to_str(), &file_hash),
            "SELECT id FROM files WHERE hash = ?1",
            (&file_hash,),
        )?;
        let track_file = TrackFile {
            id: file_id,
            name: file_name.to_str().unwrap().to_string(),
            hash: file_hash,
        };

        // Insert lyrics into table "lyrics".
        let lyrics = if let Some(new_lyrics) = &new_track.metadata.lyrics {
            let lyrics_hash = common::compute_data_hash(&new_lyrics.data);
            let lyrics_id = self.insert_or_get_id(
                "INSERT OR IGNORE INTO lyrics (hash, data) VALUES (?1, ?2)",
                (&lyrics_hash, &new_lyrics.data),
                "SELECT id FROM lyrics WHERE hash = ?1",
                (&lyrics_hash,),
            )?;

            Some(Lyrics::from_new(new_lyrics.clone(), lyrics_id, lyrics_hash))
        } else {
            None
        };

        // Insert front cover into table "images".
        let front_cover = if let Some(new_front_cover) = &new_track.metadata.front_cover {
            let front_cover_hash = common::compute_data_hash(&new_front_cover.data);
            let front_cover_id = self.insert_or_get_id(
                "INSERT OR IGNORE INTO images (hash, data) VALUES (?1, ?2)",
                (&front_cover_hash, &new_front_cover.data),
                "SELECT id FROM images WHERE hash = ?1",
                (&front_cover_hash,),
            )?;

            Some(Image::from_new(
                new_front_cover.clone(),
                front_cover_id,
                front_cover_hash,
            ))
        } else {
            None
        };

        // Insert track into table "tracks".
        self.conn
            .execute(
                "
                INSERT OR IGNORE INTO tracks (
                    title, artist, album, album_artist,
                    recording_date, release_date, track_number, disc_number,
                    genre, composer, lyricist, lyrics_id, front_cover_id, file_id
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)
                ",
                (
                    &new_track.metadata.core.title,
                    &new_track.metadata.core.artist,
                    &new_track.metadata.core.album,
                    &new_track.metadata.core.album_artist,
                    &new_track.metadata.core.recording_date,
                    &new_track.metadata.core.release_date,
                    &new_track.metadata.core.track_number,
                    &new_track.metadata.core.disc_number,
                    &new_track.metadata.core.genre,
                    &new_track.metadata.core.composer,
                    &new_track.metadata.core.lyricist,
                    lyrics.as_ref().map(|lrc| lrc.id),
                    front_cover.as_ref().map(|fc| fc.id),
                    track_file.id,
                ),
            )
            .context("rusqlite inserts track failed")?;

        Ok(Track::from_new(
            new_track,
            self.conn.last_insert_rowid(),
            lyrics,
            front_cover,
            track_file,
        ))
    }

    pub fn list(&self, filter: &TrackFilter) -> Result<Vec<TrackRow>> {
        let (where_clause, params) = Self::build_where_clause(filter);

        let mut stmt = self
            .conn
            .prepare(&format!("SELECT * FROM tracks {where_clause}"))?;
        let rows = stmt.query_map(params_from_iter(params), TrackRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    pub fn remove(&mut self, filter: &TrackFilter) -> Result<Vec<TrackRow>> {
        let (where_clause, params) = Self::build_where_clause(filter);

        let mut stmt = self
            .conn
            .prepare(&format!("DELETE FROM tracks {where_clause} RETURNING *"))?;
        let rows = stmt.query_map(params_from_iter(params), TrackRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    pub fn remove_by_id(&mut self, table_name: &str, ids: &[i64]) -> Result<i64> {
        ensure!(Self::check_table_name(table_name));

        let place_holders = std::iter::repeat_n("?", ids.len())
            .collect::<Vec<_>>()
            .join(", ");
        let rows_count = self.conn.execute(
            &format!("DELETE FROM {table_name} WHERE id IN ({place_holders})"),
            params_from_iter(ids),
        )?;

        Ok(rows_count as i64)
    }

    pub fn update(
        &mut self,
        filter: &TrackFilter,
        cols_patched: &mut HashMap<String, Patch>,
    ) -> Result<Vec<TrackRow>> {
        // BLOB fields are special: paths to new files are given, and we need
        // to insert the files into corresponding tables and update blob ids of
        // selected rows in table "tracks".
        for table_name in ["lyrics", "front_cover"] {
            if let Some(Patch::Set(path)) = cols_patched.get(table_name) {
                let data = fs::read(path)?;
                let hash = common::compute_data_hash(&data);
                let id = self.insert_or_get_id(
                    &format!("INSERT OR IGNORE INTO {table_name} (hash, data) VALUES (?, ?)"),
                    (&hash, &data),
                    &format!("SELECT id FROM {table_name} WHERE hash = ?"),
                    (&hash,),
                )?;

                cols_patched.remove(table_name);
                cols_patched.insert(format!("{table_name}_id"), Patch::Set(format!("{id}")));
            }
        }

        if let Some(Patch::Set(src_path)) = cols_patched.get("track_file") {
            let file_name = Path::new(src_path)
                .file_name()
                .with_context(|| format!("Path {src_path:?} does not exist or is not a file"))?
                .to_str()
                .context("OsStr to &str failed")?;
            let mut dst_path = Path::new("media/").join(file_name);
            // Rename the copied file to avoid overwritten file already under
            // `media/` with the same name.
            if dst_path.try_exists()? {
                let stem = dst_path
                    .file_stem()
                    .and_then(|s| s.to_str())
                    .context("OsStr to &str failed")?;
                let ext = dst_path.extension().and_then(|s| s.to_str());
                let new_name = match ext {
                    Some(ext) => format!("{stem}(1).{ext}"),
                    None => format!("{stem}(1)"),
                };
                dst_path.set_file_name(new_name);
            }

            common::atomic_copy(src_path, &dst_path)?;
            let track_hash = common::compute_file_hash(&dst_path)?;

            let file_id = self.insert_or_get_id(
                "INSERT OR IGNORE INTO files (name, hash) VALUES (?, ?)",
                (file_name, &track_hash),
                "SELECT id FROM files WHERE hash = ?",
                (&track_hash,),
            )?;

            cols_patched.remove("track_file");
            cols_patched.insert("file_id".to_string(), Patch::Set(format!("{file_id}")));
        }

        let (where_clause, where_params) = Self::build_where_clause(filter);
        let (set_clause, set_params) = Self::build_set_clause(cols_patched)?;
        let mut params = vec![];
        params.extend(set_params);
        params.extend(where_params);

        let mut stmt = self.conn.prepare(&format!(
            "UPDATE tracks {set_clause} {where_clause} RETURNING *"
        ))?;
        let rows = stmt.query_map(params_from_iter(params), TrackRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    fn insert_or_get_id(
        &self,
        insert_sql: &str,
        insert_params: impl Params,
        select_sql: &str,
        select_params: impl Params,
    ) -> Result<i64> {
        let rows_changed = self.conn.execute(insert_sql, insert_params)?;
        if rows_changed != 0 {
            Ok(self.conn.last_insert_rowid())
        } else {
            Ok(self
                .conn
                .query_one(select_sql, select_params, |row| row.get("id"))?)
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
        ["tracks", "lyrics", "images", "files"].contains(&table_name)
    }
}
