//! Implements the business logic of the subcommand.

mod add;
mod init;
mod list;
mod remove;
mod update;

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Ok, Result, anyhow, ensure};
use clap::Parser;

use crate::{
    cli::{Cli, Commands},
    models::{Patch, TrackFilter},
};

/// Parses CLI arguments and dispatches to the selected subcommand handler.
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.cmd {
        Commands::Init { force: forced } => init::execute(forced),
        Commands::List {
            json: in_json,
            filter_args,
            columns,
        } => list::execute(in_json, TrackFilter::from(*filter_args), columns),
        Commands::Add { paths, recursive } => add::execute(paths, recursive),
        Commands::Remove {
            filter_args,
            detailed,
        } => remove::execute(TrackFilter::from(*filter_args), detailed),
        Commands::Update {
            detailed,
            filter_args,
            kvs_to_set,
            columns_to_clear,
        } => {
            let columns_patched = parse_update_kvs(kvs_to_set, columns_to_clear)?;
            update::execute(TrackFilter::from(*filter_args), columns_patched, detailed)
        }
    }
}

/// Do name validation checks for columns.
fn validate_columns(cols_given: &[String], cols_valid: &[&str]) -> Result<()> {
    let cols_valid: HashSet<&str> = cols_valid.iter().copied().collect();
    for col in cols_given {
        ensure!(
            cols_valid.contains(&col.as_str()),
            "Parsed invalid column name {col}",
        );
    }
    Ok(())
}

/// Deduplicates parsed columns.
fn dedup_columns(cols: &mut Vec<String>) {
    let mut cols_filtered = HashSet::new();
    cols.retain(|col| cols_filtered.insert(col.clone()));
}

/// Parses CLI input about update into a [`HashMap<String, Patch>`].
///
/// Each valid column is mapped to a [`Patch`] variant:
///
/// - [`Patch::Set`] if the column appears in `kvs_to_set`;
/// - [`Patch::Clear`] if the column appears in `cols_to_clear`;
/// - [`Patch::Keep`] otherwise.
///
/// # Errors
///
/// Returns an error if:
///
/// - A value of `kvs_to_set` does not match the pattern `"key=value"`;
/// - A column name in `kvs_to_set` or `cols_to_clear` is not a valid field;
/// - A column is both set and cleared at the same time;
/// - A column that cannot be cleared (e.g. `title`, `track_file`) appears in
///   `cols_to_clear`.
fn parse_update_kvs(
    kvs_to_set: Vec<String>,
    cols_to_clear: Vec<String>,
) -> Result<HashMap<String, Patch>> {
    let kvs_to_set: HashMap<_, _> = kvs_to_set
        .iter()
        .map(|s| -> Result<_> {
            s.split_once('=').context(format!(
                "Param --set={} does not match the pattern \"key=value\"",
                s,
            ))
        })
        .collect::<Result<_>>()?;

    // Valid columns and for each whether it can be cleared.
    [
        ("title", false),
        ("artist", true),
        ("album", true),
        ("album_artist", true),
        ("recording_date", true),
        ("release_date", true),
        ("track_number", true),
        ("disc_number", true),
        ("genre", true),
        ("composer", true),
        ("lyricist", true),
        ("lyrics", true),
        ("front_cover", true),
        ("track_file", false),
    ]
    .into_iter()
    .map(|(col, allow_clear)| {
        let to_clear = cols_to_clear.contains(&col.to_string());
        match (kvs_to_set.get(col), to_clear) {
            (Some(_), true) => Err(anyhow!(
                "Field {} cannot be set and cleared at the same time",
                col,
            )),
            (Some(val), false) => Ok((col.to_string(), Patch::Set(val.to_string()))),
            (None, true) if !allow_clear => Err(anyhow!("Field {} cannot be cleared", col)),
            (None, true) => Ok((col.to_string(), Patch::Clear)),
            (None, false) => Ok((col.to_string(), Patch::Keep)),
        }
    })
    .collect()
}
