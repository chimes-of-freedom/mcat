//! CLI argument definitions and command-line parsing structures.

use chrono::NaiveDate;
use clap::{ArgGroup, Parser, Subcommand};

use std::{path::PathBuf, str::FromStr};

use crate::{errors::McatError, models::TrackFilter};

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
            .args([
                "titles",
                "artists",
                "albums",
                "album_artists",
                "dates",
                "track_numbers",
                "disc_numbers",
                "genres",
                "composers",
                "lyricists",
                "hashes",
            ]),
    ),
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

    /// Recording / Release date filter (repeatable).
    #[arg(long = "date")]
    pub dates: Vec<String>,

    /// Track number filter (repeatable).
    #[arg(long = "track-number")]
    pub track_numbers: Vec<usize>,

    /// Disc number filter (repeatable).
    #[arg(long = "disc-number")]
    pub disc_numbers: Vec<usize>,

    /// Genre filter (repeatable).
    #[arg(long = "genre")]
    pub genres: Vec<String>,

    /// Composer filter (repeatable).
    #[arg(long = "composer")]
    pub composers: Vec<String>,

    /// Lyricist filter (repeatable).
    #[arg(long = "lyricist")]
    pub lyricists: Vec<String>,

    /// File hash filter (repeatable).
    #[arg(long = "hash")]
    pub hashes: Vec<String>,
}

#[derive(clap::Args, Debug, Clone)]
#[command(
    group(
        ArgGroup::new("edit_group")
            .required(true)
            .args([
                "title",
                "artist",
                "album",
                "album_artist",
                "date",
                "track_number",
                "disc_number",
                "genre",
                "composer",
                "lyricist",
                "front_cover",
            ]),
    ),
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

    /// New recording / release date.
    #[arg(long)]
    pub date: Option<NaiveDate>,

    /// New track number.
    #[arg(long)]
    pub track_number: Option<usize>,

    /// New disc number.
    #[arg(long)]
    pub disc_number: Option<usize>,

    /// New genre.
    #[arg(long)]
    pub genre: Option<String>,

    /// New composer.
    #[arg(long)]
    pub composer: Option<String>,

    /// New lyricist.
    #[arg(long)]
    pub lyricist: Option<String>,

    /// Path to new lyrics text file.
    #[arg(long)]
    pub lyrics: Option<PathBuf>,

    /// Path to new front cover image file.
    #[arg(long)]
    pub front_cover: Option<PathBuf>,
}

impl TryFrom<FilterArgs> for TrackFilter {
    type Error = McatError;

    fn try_from(f: FilterArgs) -> Result<Self, Self::Error> {
        let dates = f
            .dates
            .into_iter()
            .map(|s| NaiveDate::from_str(&s))
            .collect::<Result<_, chrono::ParseError>>()?;

        Ok(TrackFilter::new(
            f.titles,
            f.artists,
            f.albums,
            f.album_artists,
            dates,
            f.track_numbers,
            f.disc_numbers,
            f.genres,
            f.composers,
            f.lyricists,
            f.hashes,
        ))
    }
}
