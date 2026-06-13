//! Add tracks into the repository. Multiple tracks can be specified at a time.

use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, ensure};
use lofty::{prelude::*, probe::Probe};
use rusqlite::Connection;

use crate::{models::NewTrack, repos::TrackRepo};

pub fn execute(paths: Vec<PathBuf>, recursive: bool) -> Result<()> {
    let mut conn = Connection::open(".mcat/track_repo.sqlite")?;
    let tx = conn.transaction()?;

    let mut new_tracks = vec![];
    let mut paths_ignored = vec![];

    for path in paths {
        let (tracks, ignored) = add_tracks(path, recursive)?;
        new_tracks.extend(tracks);
        paths_ignored.extend(ignored);
    }
    let new_tracks_len = new_tracks.len();

    for new_track in new_tracks {
        TrackRepo::insert(&tx, new_track)?;
    }

    tx.commit().context("Committing transaction (add) failed")?;

    println!(
        "{} tracks added, {} paths omitted.",
        new_tracks_len,
        paths_ignored.len(),
    );

    Ok(())
}

fn add_tracks(path: impl AsRef<Path>, recursive: bool) -> Result<(Vec<NewTrack>, Vec<PathBuf>)> {
    let path = path.as_ref();
    if !recursive || path.is_file() {
        ensure!(
            !path.is_dir(),
            "Path {path:?} is a directory. Option `-r` needed",
        );

        return Ok(
            if let Ok(tagged_file) = Probe::open(path)?.read()
                && let Some(tag) = tagged_file.primary_tag().or(tagged_file.first_tag())
            {
                (vec![NewTrack::from_tag(tag, path)], vec![])
            } else {
                (vec![], vec![PathBuf::from(path)])
            },
        );
    }

    let mut new_tracks = vec![];
    let mut paths_ignored = vec![];

    for file in fs::read_dir(path)?.flatten() {
        let (tracks, ignored) = add_tracks(file.path(), recursive)?;
        new_tracks.extend(tracks);
        paths_ignored.extend(ignored);
    }

    Ok((new_tracks, paths_ignored))
}
