//! Binary entry point for the mcat command-line application.

use mcat::{commands, config};

fn main() {
    if let Err(e) = config::init(None) {
        eprintln!("Error: {e}");

        std::process::exit(1);
    }

    if let Err(e) = commands::run() {
        eprintln!("Error: {e}");

        std::process::exit(1);
    }
}
