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
    #[command(group(
        ArgGroup::new("display_filter")
            .required(true)
            .args(["titles", "artists", "albums", "album_artists", "genres", "hashes"])
    ))]
    Display {
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
    },

    /// Edits metadata of a track.
    #[command(group(
        ArgGroup::new("edit_group")
            .required(true)
            .args(["title", "artist", "album", "album_artist", "genre", "front_cover"])
    ))]
    Edit {
        /// Hash or title of rack to edit.
        track: String,

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

        /// Path to new front cover.
        #[arg(long)]
        front_cover: Option<PathBuf>,
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
