//! Command module namespace for all subcommand handlers.

use clap::Parser;

use crate::cli::{Cli, Commands};
use crate::errors::McatResult;
use crate::models::TrackFilter;

pub mod check;
pub mod display;
pub mod edit;
pub mod export;
pub mod import;
pub mod init;
pub mod remove;

/// Parses CLI arguments and dispatches to the selected subcommand handler.
///
/// # Errors
///
/// Returns any error produced by the selected command execution.
pub fn run() -> McatResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::execute(),

        Commands::Check {
            track,
            exist,
            repair,
            save_to,
        } => check::execute(track, exist, repair, save_to),

        Commands::Remove {
            filter,
            remove_file,
        } => remove::execute(TrackFilter::from(filter), remove_file),

        Commands::Display { filter } => display::execute(TrackFilter::from(filter)),

        Commands::Edit { track, edit } => edit::execute(track, edit),

        Commands::Import { path, move_files } => import::execute(path, move_files),
    }
}
