//! Remove tracks in the repository. Offer options to specify filter conditions.

use anyhow::Result;
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

    let conn = Connection::open(".mcat/track_repo.sqlite")?;
    let mut track_repo = TrackRepo::new(&conn);
    let tracks_removed = track_repo.remove(&filter)?;

    if detailed {
        println!("{}", serde_json::to_string_pretty(&tracks_removed)?);
    } else {
        println!("{} tracks removed.", tracks_removed.len());
    }

    Ok(())
}
