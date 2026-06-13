//! List tracks in the repository. Offer options to specify filter conditions.

use anyhow::Result;
use rusqlite::Connection;

use crate::{
    commands::{dedup_columns, validate_columns},
    common,
    models::TrackFilter,
    repos::TrackRepo,
};

pub fn execute(in_json: bool, filter: TrackFilter, mut cols: Vec<String>) -> Result<()> {
    // Preprocess columns given.
    validate_columns(&cols, common::CANONICAL_COLUMNS)?;
    dedup_columns(&mut cols);

    let conn = Connection::open(".mcat/track_repo.sqlite")?;

    let track_rows = TrackRepo::list(&conn, &filter)?;
    if cols.is_empty() {
        cols = vec!["id", "title", "artist", "album"]
            .into_iter()
            .map(str::to_string)
            .collect();
    }
    let rows_reflected = common::reflect_rows(&track_rows, &cols)?;

    if in_json {
        println!("{}", serde_json::to_string_pretty(&rows_reflected)?);
    } else {
        let table = common::build_table(&rows_reflected, &cols);
        println!("{}", table);
    }

    Ok(())
}
