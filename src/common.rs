//! Commonly used functions.

use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use indexmap::IndexMap;
use serde::Serialize;
use serde_json::Value;
use tabled::{Table, builder::Builder, settings::Style};
use tempfile::NamedTempFile;

pub const CANONICAL_COLUMNS: &[&str] = &[
    "id",
    "title",
    "artist",
    "album",
    "album_artist",
    "recording_date",
    "release_date",
    "track_number",
    "disc_number",
    "genre",
    "composer",
    "lyricist",
    "lyrics_id",
    "front_cover_id",
    "file_id",
];

/// Computes BLAKE3 hash of given `&[u8]`.
pub fn compute_data_hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    hasher.finalize().as_bytes().to_vec()
}

/// A convenient function to compute BLAKE3 hash of given file.
/// Calls [`compute_data_hash`] inside the function.
pub fn compute_file_hash(path: impl AsRef<Path>) -> Result<Vec<u8>> {
    let data = fs::read(path)?;
    Ok(compute_data_hash(&data))
}

/// Atomicly copies a file, which means there will be no target file
/// (but may leave a temp file if cleanup fails) if copy fails.
/// Does nothing if `src_path == dst_path`.
pub fn atomic_copy(src_path: impl AsRef<Path>, dst_path: impl AsRef<Path>) -> Result<()> {
    let src_path = src_path.as_ref();
    let dst_path = dst_path.as_ref();
    if src_path == dst_path {
        return Ok(());
    }

    (|| {
        let dst_dir = dst_path
            .parent()
            .with_context(|| format!("The destination {} has no parent", dst_path.display()))?;
        let tmp_file = NamedTempFile::new_in(dst_dir)?;

        fs::copy(src_path, tmp_file.path())?;
        tmp_file.persist(dst_path)?;

        anyhow::Ok(())
    })()
    .with_context(|| format!("Copying file {} to media/ failed", src_path.display()))?;

    Ok(())
}

/// Converts rows to vector of [`IndexMap<String, Value>`].
pub fn reflect_rows(
    rows: &[impl Serialize],
    cols: &[String],
) -> Result<Vec<IndexMap<String, Value>>> {
    let Value::Array(rows) = serde_json::to_value(rows)? else {
        bail!("FATAL: Converting rows to `Vec<Value>` failed");
    };

    // Remove key-value pairs if the key is not in selected columns.
    Ok(rows
        .into_iter()
        .filter_map(|row| match row {
            Value::Object(row) => Some(
                CANONICAL_COLUMNS
                    .iter()
                    .filter(|col| cols.contains(&col.to_string()))
                    .filter_map(|&col| row.get(col).map(|val| (col.to_string(), val.clone())))
                    .collect(),
            ),
            _ => None,
        })
        .collect())
}

/// Build a table from rows of [`IndexMap<String, Value>`] and column names.
pub fn build_table(rows: &[IndexMap<String, Value>], cols: &[String]) -> Table {
    let mut builder = Builder::default();

    builder.push_record(cols);
    for row in rows {
        let row: Vec<_> = cols
            .iter()
            .map(|col| {
                row.get(col)
                    .map(|v| {
                        // `Value::Null` to empty string
                        if v.is_null() {
                            String::new()
                        } else {
                            // Ensure a string value is not parsed to a string
                            // with quote marks.
                            v.as_str().unwrap_or(&v.to_string()).to_string()
                        }
                    })
                    .unwrap_or_default()
            })
            .collect();
        builder.push_record(row);
    }

    let mut table = builder.build();
    table.with(Style::modern());
    table
}
