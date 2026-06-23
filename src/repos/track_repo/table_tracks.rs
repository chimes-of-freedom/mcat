//! Functions associated with table "tracks".

use std::{collections::HashMap, fs, path::Path};

use anyhow::{Context, Result, ensure};
use rusqlite::{Connection, Transaction, params_from_iter};

use crate::{
    common,
    models::{NewTrack, NewTrackFile, Patch, Track, TrackFilter, TrackRow},
    repos::{
        TrackRepo,
        track_repo::{
            table_images::TableImages, table_lyrics::TableLyrics,
            table_track_files::TableTrackFiles,
        },
    },
};

pub struct TableTracks;

impl TableTracks {
    pub(super) fn insert_one(tx: &Transaction, new_track: NewTrack) -> Result<Track> {
        let file_name = new_track
            .file
            .path
            .file_name()
            .with_context(|| format!("Path {:?} terminates in ..", new_track.file.path))?;
        let file_path = Path::new("media").join(file_name);

        // Copy file to "media/".
        common::atomic_copy(&new_track.file.path, &file_path)?;

        // Insert `TrackFile` into table "track_files".
        let track_file = TableTrackFiles::insert_one(tx, NewTrackFile { path: file_path })?;

        // Insert lyrics into table "lyrics".
        let lyrics = new_track
            .metadata
            .lyrics
            .as_ref()
            .map(|lrc| TableLyrics::insert_one(tx, lrc.clone()))
            .transpose()?;

        // Insert front cover into table "images".
        let front_cover = new_track
            .metadata
            .front_cover
            .as_ref()
            .map(|fc| TableImages::insert_one(tx, fc.clone()))
            .transpose()?;

        // Insert track into table "tracks".
        tx.execute(
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
        .with_context(|| {
            format!(
                "rusqlite inserts track failed. track path: {}",
                new_track.file.path.display(),
            )
        })?;

        Ok(Track::from_new(
            new_track,
            tx.last_insert_rowid(),
            lyrics,
            front_cover,
            track_file,
        ))
    }

    pub(super) fn select_by_filter(
        conn: &Connection,
        filter: &TrackFilter,
    ) -> Result<Vec<TrackRow>> {
        let (where_clause, params) = TrackRepo::build_where_clause(filter);

        let mut stmt = conn.prepare(&format!("SELECT * FROM tracks {where_clause}"))?;
        let rows = stmt.query_map(params_from_iter(params), TrackRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    pub(super) fn remove_by_filter(
        tx: &Transaction,
        filter: &TrackFilter,
    ) -> Result<Vec<TrackRow>> {
        let (where_clause, params) = TrackRepo::build_where_clause(filter);

        let mut stmt = tx.prepare(&format!("DELETE FROM tracks {where_clause} RETURNING *"))?;
        let rows = stmt.query_map(params_from_iter(params), TrackRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }

    pub(super) fn remove_by_id(tx: Transaction, table_name: &str, ids: &[i64]) -> Result<i64> {
        ensure!(TrackRepo::check_table_name(table_name));

        let place_holders = std::iter::repeat_n("?", ids.len())
            .collect::<Vec<_>>()
            .join(", ");
        let rows_count = tx.execute(
            &format!("DELETE FROM {table_name} WHERE id IN ({place_holders})"),
            params_from_iter(ids),
        )?;

        Ok(rows_count as i64)
    }

    pub(super) fn update_by_filter(
        tx: &Transaction,
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
                let id = TrackRepo::insert_or_get_id(
                    tx,
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

            let file_id = TrackRepo::insert_or_get_id(
                tx,
                "INSERT OR IGNORE INTO track_files (name, hash) VALUES (?, ?)",
                (file_name, &track_hash),
                "SELECT id FROM track_files WHERE hash = ?",
                (&track_hash,),
            )?;

            cols_patched.remove("track_file");
            cols_patched.insert("file_id".to_string(), Patch::Set(format!("{file_id}")));
        }

        let (where_clause, where_params) = TrackRepo::build_where_clause(filter);
        let (set_clause, set_params) = TrackRepo::build_set_clause(cols_patched)?;
        let mut params = vec![];
        params.extend(set_params);
        params.extend(where_params);

        let mut stmt = tx.prepare(&format!(
            "UPDATE tracks {set_clause} {where_clause} RETURNING *"
        ))?;
        let rows = stmt.query_map(params_from_iter(params), TrackRow::from_row)?;

        Ok(rows.collect::<rusqlite::Result<Vec<_>>>()?)
    }
}
