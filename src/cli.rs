//! CLI argument definitions and command-line parsing structures.

use clap::{ArgGroup, Parser, Subcommand};

use std::path::PathBuf;

/// Top-level CLI parser for mcat.
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Selected subcommand action.
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported mcat subcommands.
#[derive(Subcommand)]
pub enum Commands {
    /// Displays music metadata stored in the repository.
    Display,

    /// Edits metadata for a music file.
    Edit {
        /// Path to the music file to edit.
        src: PathBuf,

        /// New title.
        #[arg(long)]
        title: Option<String>,

        /// New artist.
        #[arg(long)]
        artist: Option<String>,

        /// New album.
        #[arg(long)]
        album: Option<String>,

        /// New album artist.
        #[arg(long)]
        album_artist: Option<String>,

        /// New genre.
        #[arg(long)]
        genre: Option<String>,

        /// Output path for the edited file (defaults to `src`).
        #[arg(long = "output", short = 'o')]
        dst: Option<PathBuf>,
    },

    /// Initializes a repository from files under `media/`.
    Init,

    /// Checks consistency between files under `media/` and repository records.
    Check {
        /// Checks only whether files under `media/` are tracked.
        #[arg(group = "filter", short, long, default_value = "false")]
        track: bool,

        /// Checks only whether tracked files still exist under `media/`.
        #[arg(group = "filter", short, long, default_value = "false")]
        exist: bool,

        /// Repairs repository state according to check results.
        #[arg(short, long, default_value = "false")]
        repair: bool,

        /// Saves check results as TOML.
        #[arg(short, long)]
        save_to: Option<PathBuf>,
    },

    /// Removes tracks from the repository, optionally removing files.
    #[command(group(
        ArgGroup::new("remove_filter")
            .required(true)
            .args(["titles", "artists", "albums", "album_artists", "genres", "hashes"])
    ))]
    Remove {
        /// Track title filter (repeatable).
        #[arg(long = "title")]
        titles: Vec<String>,

        /// Track artist filter (repeatable).
        #[arg(long = "artist")]
        artists: Vec<String>,

        /// Album title filter (repeatable).
        #[arg(long = "album")]
        albums: Vec<String>,

        /// Album artist filter (repeatable).
        #[arg(long = "album-artist")]
        album_artists: Vec<String>,

        /// Genre filter (repeatable).
        #[arg(long = "genre")]
        genres: Vec<String>,

        /// File hash filter (repeatable).
        #[arg(long = "hash")]
        hashes: Vec<String>,

        /// Removes the media file as well.
        #[arg(short, long, default_value = "false")]
        remove_file: bool,
    },
}
