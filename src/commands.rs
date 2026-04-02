//! Command module namespace for all subcommand handlers.

use clap::Parser;

use crate::cli::{Cli, Commands};
use mcat::errors::McatResult;

pub mod check;
pub mod display;
pub mod edit;
pub mod export;
pub mod import;
pub mod init;
pub mod remove;

pub fn run() -> McatResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => init::execute(),

        Commands::Check {
            track,
            exist,
            repair,
            save_to,
        } => check::execute(track, exist, repair, save_to),

        Commands::Remove { track, remove_file } => remove::execute(track, remove_file),

        _ => todo!("This subcommand is not implemented yet."),
    }
}
