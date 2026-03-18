use std::path::PathBuf;

use clap::{Parser, Subcommand};

use mcat::{TagAttributes, utils};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// action
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Display {
        /// path of music file to be displayed
        #[arg(long)]
        path: PathBuf,
    },
    Edit {
        /// path of music file to be edited
        #[arg(long)]
        src: PathBuf,

        /// title
        #[arg(long)]
        title: Option<String>,

        /// artist
        #[arg(long)]
        artist: Option<String>,

        /// album
        #[arg(long)]
        album: Option<String>,

        /// genre
        #[arg(long)]
        genre: Option<String>,

        /// path of edited music file to be saved at
        #[arg(long)]
        dst: Option<PathBuf>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Display { path } => {
            let primary_tag = match utils::get_primary_tag(path) {
                Ok(tag) => tag,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            utils::display_tag(&primary_tag);
        }

        Commands::Edit {
            src: src_path,
            dst: dst_path,
            title,
            artist,
            album,
            genre,
        } => {
            let mut primary_tag = match utils::get_primary_tag(&src_path) {
                Ok(tag) => tag,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            // copy `src_path` to `dst_path` if not the same
            let output_path = if let Some(path) = dst_path {
                if let Err(e) = std::fs::copy(&src_path, &path) {
                    eprintln!("Failed to copy file: {:?}", e);
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
                genre,
            };
            if let Err(e) = utils::edit_tag(&output_path, &mut primary_tag, tag_attrs) {
                eprintln!("Error: {:?}", e);
                std::process::exit(1);
            }

            println!("Entered edit mode. Output path: {:?}", &output_path);
        }
    }
}
