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
    Display {
        /// path of music file to display
        path: PathBuf,
    },

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
}
