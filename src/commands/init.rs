//! `init` command handler for initializing repository metadata state.

use crate::errors::McatResult;
use crate::repos::{Repository, toml_repo::Database};
use crate::services::scan_media;

use std::path::Path;

pub fn execute() -> McatResult<()> {
    let mut repo = Database::init_empty();
    scan_media(&mut repo, Path::new("media/"), false)?;
    repo.persist()
}
