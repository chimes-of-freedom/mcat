//! Initializes the repository.

use std::{collections::HashSet, fs, path::Path};

use anyhow::{Result, ensure};
use lofty::{prelude::*, probe::Probe};
use rusqlite::Connection;

use crate::{models::NewTrack, repos::TrackRepo};

pub fn execute(forced: bool) -> Result<()> {
    // If `.mcat/` exists, ensure we're in force mode and remove the directory,
    // otherwise return an error.
    if Path::new(".mcat").try_exists()? {
        ensure!(forced, "Directory .mcat already exists");
        fs::remove_dir_all(".mcat")?;
    }
    fs::create_dir(".mcat")?;

    let conn = Connection::open(".mcat/track_repo.sqlite")?;
    let mut track_repo = TrackRepo::new(&conn);

    track_repo.init()?;

    let mut cores_filtered = HashSet::new();
    for file in fs::read_dir("media")? {
        if let Ok(file) = file
            && file.file_type().is_ok_and(|file_type| file_type.is_file())
            && let Ok(tagged_file) = Probe::open(file.path())?.read()
            && let Some(tag) = tagged_file.primary_tag().or(tagged_file.first_tag())
        {
            let new_track = NewTrack::from_tag(tag, file.path());
            if cores_filtered.insert(new_track.metadata.core.clone()) {
                track_repo.insert(new_track)?;
            }
        }
    }

    Ok(())
}
