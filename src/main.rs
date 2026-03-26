use mcat::*;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// action
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
        #[arg(long = "album-artist")]
        album_artist: Option<String>,

        /// new genre
        #[arg(long)]
        genre: Option<String>,

        /// path of edited music file to be saved at (default `src`)
        #[arg(long = "output", short = 'o')]
        dst: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Display { path } => {
            let primary_tag = match common::get_primary_tag(path) {
                Ok(tag) => tag,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            display::display_tag(&primary_tag);
        }

        Commands::Edit {
            src: src_path,
            dst: dst_path,
            title,
            artist,
            album,
            album_artist,
            genre,
        } => {
            let mut primary_tag = match common::get_primary_tag(&src_path) {
                Ok(tag) => tag,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            // copy `src_path` to `dst_path` if not the same
            let output_path = if let Some(path) = dst_path {
                if let Err(e) = std::fs::copy(&src_path, &path) {
                    eprintln!("Failed to copy file: {}", e);
                    std::process::exit(1);
                }
                path
            } else {
                src_path
            };

            let tag_attrs = TagAttributes {
                title,
                artist,
                album,
                album_artist,
                genre,
            };
            if let Err(e) = edit::edit_tag(&output_path, &mut primary_tag, tag_attrs) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }

            println!("Edit done.");
            display::display_tag(&primary_tag);
        }
    }
}
