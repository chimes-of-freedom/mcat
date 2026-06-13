//! Update fields of tracks in the repository. Options are provided to specify
//! filter conditions.

use std::collections::HashMap;

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::{
    models::{Patch, TrackFilter},
    repos::TrackRepo,
};

pub fn execute(
    filter: TrackFilter,
    mut cols_patched: HashMap<String, Patch>,
    detailed: bool,
) -> Result<()> {
    if filter.is_empty() {
        println!(
            "{}",
            if detailed {
                serde_json::to_string_pretty(&serde_json::json!([]))?
            } else {
                "0 tracks updated.".to_string()
            },
        );
        return Ok(());
    }

    let mut conn = Connection::open(".mcat/track_repo.sqlite")?;
    let tx = conn.transaction()?;
    let tracks_updated = TrackRepo::update(&tx, &filter, &mut cols_patched)?;

    if detailed {
        println!("{}", serde_json::to_string_pretty(&tracks_updated)?);
    } else {
        println!("{} tracks updated.", tracks_updated.len());
    }

    tx.commit()
        .context("Committing transaction (update) failed")
}
