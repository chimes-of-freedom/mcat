//! Binary entry point for the mcat command-line application.

mod cli;
mod commands;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("Error: {e}");
    }
}
