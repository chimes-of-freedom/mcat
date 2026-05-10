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
        #[arg(value_name = "track")]
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
        #[arg(short, long, value_name = "save-path")]
        save_path: Option<PathBuf>,
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
        #[arg(value_name = "path")]
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
    /// Track title filter.
    #[arg(long = "title", value_name = "title")]
    pub titles: Vec<String>,

    /// Track artist filter.
    #[arg(long = "artist", value_name = "artist")]
    pub artists: Vec<String>,

    /// Album title filter.
    #[arg(long = "album", value_name = "album")]
    pub albums: Vec<String>,

    /// Album artist filter.
    #[arg(long = "album-artist", value_name = "album-artist")]
    pub album_artists: Vec<String>,

    /// Recording / Release date filter.
    #[arg(long = "date", value_name = "date")]
    pub dates: Vec<String>,

    /// Track number filter.
    #[arg(long = "track-number", value_name = "track-number")]
    pub track_numbers: Vec<usize>,

    /// Disc number filter.
    #[arg(long = "disc-number", value_name = "disc-number")]
    pub disc_numbers: Vec<usize>,

    /// Genre filter.
    #[arg(long = "genre", value_name = "genre")]
    pub genres: Vec<String>,

    /// Composer filter.
    #[arg(long = "composer", value_name = "composer")]
    pub composers: Vec<String>,

    /// Lyricist filter.
    #[arg(long = "lyricist", value_name = "lyricist")]
    pub lyricists: Vec<String>,

    /// File hash filter.
    #[arg(long = "hash", value_name = "hash")]
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
    #[arg(long, value_name = "title")]
    pub title: Option<String>,

    /// New artist.
    #[arg(long, value_name = "artist")]
    pub artist: Option<String>,

    /// New album.
    #[arg(long, value_name = "album")]
    pub album: Option<String>,

    /// New album artist.
    #[arg(long, value_name = "album-artist")]
    pub album_artist: Option<String>,

    /// New recording / release date.
    #[arg(long, value_name = "date")]
    pub date: Option<NaiveDate>,

    /// New track number.
    #[arg(long, value_name = "track-number")]
    pub track_number: Option<usize>,

    /// New disc number.
    #[arg(long, value_name = "disc-number")]
    pub disc_number: Option<usize>,

    /// New genre.
    #[arg(long, value_name = "genre")]
    pub genre: Option<String>,

    /// New composer.
    #[arg(long, value_name = "composer")]
    pub composer: Option<String>,

    /// New lyricist.
    #[arg(long, value_name = "lyricist")]
    pub lyricist: Option<String>,

    /// Path to new lyrics text file.
    #[arg(long, value_name = "lyrics")]
    pub lyrics: Option<PathBuf>,

    /// Path to new front cover image file.
    #[arg(long, value_name = "front-cover")]
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
