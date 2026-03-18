use std::path::PathBuf;

use clap::{Parser, Subcommand};

use mcat::utils::{self, display_tag};

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
        /// the path of music file to be displayed
        #[arg(short, long, value_name = "path")]
        path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Display {path} => {
            let primary_tag = match utils::get_primary_tag(path) {
                Ok(tag) => tag,
                Err(e) => {
                    eprintln!("Error: {:?}", e);
                    return;
                }
            };

            display_tag(&primary_tag);
        },
    }
}
