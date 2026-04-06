//! `init` command handler for initializing repository metadata state.

use mcat::config;
use mcat::errors::McatResult;
use mcat::repos::{Repo, toml_repo::TomlDb};
use mcat::services::scan_media;

/// Executes the `init` command.
///
/// # Errors
///
/// Returns repository persistence errors or scan failures while reading files
/// under `media/`.
pub fn execute() -> McatResult<()> {
    let mut repo: TomlDb = Repo::init_empty();
    scan_media(&mut repo, config::media_dir_path(), false)?;
    repo.persist()
}
