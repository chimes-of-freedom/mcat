//! Binary entry point for the mcat command-line application.

use mcat::{commands, config, errors::print_error_chain};

fn main() {
    if let Err(e) = config::init(None) {
        print_error_chain(&e);

        std::process::exit(1);
    }

    if let Err(e) = commands::run() {
        print_error_chain(&e);

        std::process::exit(1);
    }
}
