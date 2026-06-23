//! Remove tracks in the repository. Offer options to specify filter conditions.

use anyhow::{Context, Result};
use rusqlite::Connection;

use crate::{models::TrackFilter, repos::TrackRepo};

pub fn execute(filter: TrackFilter, detailed: bool) -> Result<()> {
    if filter.is_empty() {
        println!(
            "{}",
            if detailed {
                serde_json::to_string_pretty(&serde_json::json!([]))?
            } else {
                "0 tracks removed.".to_string()
            },
        );
        return Ok(());
    }

    let mut conn = Connection::open(".mcat/track_repo.sqlite")?;
    let tx = conn.transaction()?;
    let tracks_removed = TrackRepo::remove_by_filter(&tx, &filter)?;

    if detailed {
        println!("{}", serde_json::to_string_pretty(&tracks_removed)?);
    } else {
        println!("{} tracks removed.", tracks_removed.len());
    }

    tx.commit()
        .context("Committing transaction (remove) failed")
}
