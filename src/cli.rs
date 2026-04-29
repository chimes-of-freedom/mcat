//! CLI argument definitions and command-line parsing structures.

use clap::{ArgGroup, Parser, Subcommand};

use std::path::PathBuf;

use crate::models::TrackFilter;

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
    Display {
        #[command(flatten)]
        filter: FilterArgs,
    },

    /// Edits metadata of a track.
    Edit {
        /// Hash or title of rack to edit.
        track: String,

        #[command(flatten)]
        edit: EditArgs,
    },

    /// Initializes a repository from files under `media/`.
    Init,

    /// Checks consistency between files under `media/` and repository records.
    Check {
        /// Checks only whether files under `media/` are tracked.
        #[arg(group = "check_filter_group", short, long, default_value = "false")]
        track: bool,

        /// Checks only whether tracked files still exist under `media/`.
        #[arg(group = "check_filter_group", short, long, default_value = "false")]
        exist: bool,

        /// Repairs repository state according to check results.
        #[arg(short, long, default_value = "false")]
        repair: bool,

        /// Saves check results as TOML.
        #[arg(short, long)]
        save_to: Option<PathBuf>,
    },

    /// Removes tracks from the repository, optionally removing files.
    Remove {
        #[command(flatten)]
        filter: FilterArgs,

        /// Removes the media file as well.
        #[arg(short, long, default_value = "false")]
        remove_file: bool,
    },

    /// Imports music files from a directory.
    Import {
        /// Path to directory.
        path: PathBuf,

        /// Move files instead of copying them.
        #[arg(short, long = "move")]
        move_files: bool,
    },
}

#[derive(clap::Args, Debug, Clone)]
#[command(
    group(
        ArgGroup::new("filter_group")
            .required(true)
            .args(["titles", "artists", "albums", "album_artists", "genres", "hashes"])
    )
)]
pub struct FilterArgs {
    /// Track title filter (repeatable).
    #[arg(long = "title")]
    pub titles: Vec<String>,

    /// Track artist filter (repeatable).
    #[arg(long = "artist")]
    pub artists: Vec<String>,

    /// Album title filter (repeatable).
    #[arg(long = "album")]
    pub albums: Vec<String>,

    /// Album artist filter (repeatable).
    #[arg(long = "album-artist")]
    pub album_artists: Vec<String>,

    /// Genre filter (repeatable).
    #[arg(long = "genre")]
    pub genres: Vec<String>,

    /// File hash filter (repeatable).
    #[arg(long = "hash")]
    pub hashes: Vec<String>,
}

#[derive(clap::Args, Debug, Clone)]
#[command(
    group(
        ArgGroup::new("edit_group")
            .required(true)
            .args(["title", "artist", "album", "album_artist", "genre", "front_cover"])
    )
)]
pub struct EditArgs {
    /// New title.
    #[arg(long)]
    pub title: Option<String>,

    /// New artist.
    #[arg(long)]
    pub artist: Option<String>,

    /// New album.
    #[arg(long)]
    pub album: Option<String>,

    /// New album artist.
    #[arg(long)]
    pub album_artist: Option<String>,

    /// New genre.
    #[arg(long)]
    pub genre: Option<String>,

    /// Path to new front cover.
    #[arg(long)]
    pub front_cover: Option<PathBuf>,
}

// cli.rs 或 commands.rs
impl From<FilterArgs> for TrackFilter {
    fn from(f: FilterArgs) -> Self {
        TrackFilter::new(
            f.titles,
            f.artists,
            f.albums,
            f.album_artists,
            f.genres,
            f.hashes,
        )
    }
}
