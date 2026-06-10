//! Implements the business logic of the subcommand.

mod add;
mod init;
mod list;
mod remove;
mod update;

use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result, ensure};
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
    let mut kvs_patched = HashMap::new();
    // Valid columns and for each whether it can be cleared.
    let cols_valid = [
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
    ];

    let kvs_to_set: HashMap<_, _> = kvs_to_set
        .iter()
        .map(|s| -> Result<_> {
            s.split_once('=').context(format!(
                "Param --set={} does not match the pattern \"key=value\"",
                s,
            ))
        })
        .collect::<Result<_>>()?;

    for (col, can_be_cleared) in cols_valid {
        let to_be_set = kvs_to_set.contains_key(&col);
        let to_be_cleared = cols_to_clear.contains(&col.to_string());

        ensure!(
            !(to_be_set && to_be_cleared),
            "Field {} cannot be set and cleared at the same time",
            col,
        );
        ensure!(
            #[allow(clippy::nonminimal_bool)]
            !(!can_be_cleared && to_be_cleared),
            "Field {} cannot be cleared",
            col
        );

        if to_be_set {
            kvs_patched.insert(
                col.to_string(),
                Patch::Set(kvs_to_set.get(col).unwrap().to_string()),
            );
        } else if to_be_cleared {
            kvs_patched.insert(col.to_string(), Patch::Clear);
        } else {
            kvs_patched.insert(col.to_string(), Patch::Keep);
        }
    }

    Ok(kvs_patched)
}
