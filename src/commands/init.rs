//! `init` command handler for initializing repository metadata state.

use mcat::errors::McatResult;
use mcat::repos::{Repo, toml_repo::TomlDb};
use mcat::services::scan_media;

use std::path::Path;

/// Executes the `init` command.
///
/// # Errors
///
/// Returns repository persistence errors or scan failures while reading files
/// under `media/`.
pub fn execute() -> McatResult<()> {
    let mut repo: TomlDb = Repo::init_empty();
    scan_media(&mut repo, Path::new("media/"), false)?;
    repo.persist()
}
