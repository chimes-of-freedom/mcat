use std::path::Path;

use clap::Parser;

use mcat::utils::{self, display_tag};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// the path of music file
    #[arg(short, long, value_name = "path")]
    path: String,
}

fn main() {
    let cli = Cli::parse();
    let path = Path::new(&cli.path);

    let primary_tag = match utils::get_primary_tag(path) {
        Ok(tag) => tag,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return;
        }
    };

    display_tag(&primary_tag);
}
