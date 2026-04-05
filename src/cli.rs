//! CLI argument definitions and command-line parsing structures.

use clap::{Parser, Subcommand};

use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// action
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// display the music metadata
    Display,

    /// write the music metadata
    Edit {
        /// path of music file to be edited
        src: PathBuf,

        /// new title
        #[arg(long)]
        title: Option<String>,

        /// new artist
        #[arg(long)]
        artist: Option<String>,

        /// new album
        #[arg(long)]
        album: Option<String>,

        /// new album artist
        #[arg(long)]
        album_artist: Option<String>,

        /// new genre
        #[arg(long)]
        genre: Option<String>,

        /// path of edited music file to be saved at (default `src`)
        #[arg(long = "output", short = 'o')]
        dst: Option<PathBuf>,
    },

    /// init a repository
    Init,

    /// check if files in `media/` are tracked by the database,
    /// or if files in the database exist in `media/`
    Check {
        /// only check if files in `media/` are tracked by the database
        #[arg(group = "filter", short, long, default_value = "false")]
        track: bool,

        /// only check if files in the database exist in `media/`
        #[arg(group = "filter", short, long, default_value = "false")]
        exist: bool,

        /// repair according to the check results
        #[arg(short, long, default_value = "false")]
        repair: bool,

        /// save results in toml
        #[arg(short, long)]
        save_to: Option<PathBuf>,
    },

    /// remove a track's metadata from the repository,
    /// along with the file if specified
    Remove {
        /// file hash or the track title
        track: String,

        /// remove the file
        #[arg(short, long, default_value = "false")]
        remove_file: bool,
    },
}
