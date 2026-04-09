//! Command module namespace for all subcommand handlers.

use clap::Parser;

use crate::cli::{Cli, Commands};
use mcat::errors::McatResult;

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
            titles,
            artists,
            albums,
            album_artists,
            genres,
            hashes,
            remove_file,
        } => remove::execute(
            titles,
            artists,
            albums,
            album_artists,
            genres,
            hashes,
            remove_file,
        ),

        Commands::Display {
            titles,
            artists,
            albums,
            album_artists,
            genres,
            hashes,
        } => display::execute(titles, artists, albums, album_artists, genres, hashes),

        Commands::Edit {
            track,
            title,
            artist,
            album,
            album_artist,
            genre,
            front_cover,
        } => edit::execute(
            track,
            title,
            artist,
            album,
            album_artist,
            genre,
            front_cover,
        ),

        Commands::Import { path, move_files } => import::execute(path, move_files),
    }
}
