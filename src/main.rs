//! Binary entry point for the mcat command-line application.

mod commands;
mod cli;

fn main() {
    if let Err(e) = commands::run() {
        eprintln!("Error: {e}");
    }
}
