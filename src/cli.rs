//! Definitions of the CLI interface.

use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

use crate::models::TrackFilter;

#[derive(Parser)]
#[command(
    version,
    about,
    long_about = "
mcat is a music cataloging tool aimed at providing a graceful
way to manage music files along with their metadata.",
    disable_help_subcommand = true
)]
pub struct Cli {
    // Subcommand.
    #[command(subcommand)]
    pub cmd: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a repository.
    Init {
        /// Overwrite .mcat/ if it already exists.
        #[arg(short, long, default_value = "false")]
        force: bool,
    },

    /// List selected tracks in the repository.
    ///
    /// By default all tracks will be listed.
    ///
    /// e.g. `mcat list --artist "Bob Dylan" --track_number 1 --track_number 2`
    /// implies a track of Bob Dylan will be listed if it's the first or the
    /// second track in an album.
    #[command(alias = "ls")]
    List {
        /// Print the list in JSON format.
        #[arg(short, long)]
        json: bool,

        #[command(flatten)]
        filter_args: Box<FilterArgs>,

        /// Add a column to the result.
        #[arg(long = "column", value_name = "column")]
        columns: Vec<String>,
    },

    /// Add tracks into the repository.
    ///
    /// Multiple tracks can be specified at a time.
    Add {
        /// Paths to track files to add. Accept directories if `-r` / `--recursive`
        /// is specified.
        paths: Vec<PathBuf>,

        /// Recursively add tracks under a directory.
        #[arg(short, long)]
        recursive: bool,
    },

    /// Remove selected tracks from the repository.
    ///
    /// By default no tracks will be removed.
    ///
    /// `mcat remove` does not removes BLOB files like lyrics, front covers and
    /// track files from the repository.
    #[command(alias = "rm")]
    Remove {
        #[command(flatten)]
        filter_args: Box<FilterArgs>,

        /// Show the information of tracks removed in JSON format.
        #[arg(short, long)]
        detailed: bool,
    },

    /// Update fields of selected tracks in the repository.
    ///
    /// By default no tracks will be updated.
    Update {
        /// Show the information of tracks updated in JSON format.
        #[arg(short, long)]
        detailed: bool,

        #[command(flatten)]
        filter_args: Box<FilterArgs>,

        /// Set the value of a field for tracks.
        ///
        /// If a BLOB field is to be updated, the value is the path to the new
        /// BLOB file.
        ///
        /// e.g. `mcat update --artist "bob dylan" --set artist="Bob Dylan"`
        /// corrects the artist name of Bob Dylan's songs, and
        /// `mcat update --title "Tempest" --set track_file="/path/to/Tempest - Bob Dylan.flac"`
        /// updates the track file of Bob Dylan's *Tempest*.
        ///
        /// Valid fields are: title, artist, album, album_artist,
        /// recording_date, release_date, track_number, disc_number, genre,
        /// composer, lyricist, front_cover, track_file.
        #[arg(long = "set", value_name = "key=value")]
        kvs_to_set: Vec<String>,

        /// Clear the value of a field for tracks.
        #[arg(long = "clear", value_name = "key")]
        columns_to_clear: Vec<String>,
    },
}

#[derive(Args)]
pub struct FilterArgs {
    /// ID of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "id", value_name = "id")]
    ids: Vec<i64>,

    /// Title of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "title", value_name = "title")]
    titles: Vec<String>,

    /// Artist of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "artist", value_name = "artist")]
    artists: Vec<String>,

    /// Album of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "album", value_name = "album")]
    albums: Vec<String>,

    /// Album artist of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "album_artist", value_name = "album_artist")]
    album_artists: Vec<String>,

    /// Recording date of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "recording_date", value_name = "recording_date")]
    recording_dates: Vec<String>,

    /// Release date of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "release_date", value_name = "release_date")]
    release_dates: Vec<String>,

    /// Track number of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "track_number", value_name = "track_number")]
    track_numbers: Vec<i64>,

    /// Disc number of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "disc_number", value_name = "disc_number")]
    disc_numbers: Vec<i64>,

    /// Genre of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "genre", value_name = "genre")]
    genres: Vec<String>,

    /// Composer of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "composer", value_name = "composer")]
    composers: Vec<String>,

    /// Lyricist of the track.
    ///
    /// Multiple values implies "OR" logic.
    #[arg(long = "lyricist", value_name = "lyricist")]
    lyricists: Vec<String>,
}

impl From<FilterArgs> for TrackFilter {
    fn from(filter_args: FilterArgs) -> Self {
        Self {
            ids: filter_args.ids.into_iter().collect(),
            titles: filter_args.titles.into_iter().collect(),
            artists: filter_args.artists.into_iter().collect(),
            albums: filter_args.albums.into_iter().collect(),
            album_artists: filter_args.album_artists.into_iter().collect(),
            recording_dates: filter_args.recording_dates.into_iter().collect(),
            release_dates: filter_args.release_dates.into_iter().collect(),
            track_numbers: filter_args.track_numbers.into_iter().collect(),
            disc_numbers: filter_args.disc_numbers.into_iter().collect(),
            genres: filter_args.genres.into_iter().collect(),
            composers: filter_args.composers.into_iter().collect(),
            lyricists: filter_args.lyricists.into_iter().collect(),
        }
    }
}
