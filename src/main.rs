//! Binary entry point for the mcat command-line application.

use mcat::commands;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("Error: {e}");
    }
}
