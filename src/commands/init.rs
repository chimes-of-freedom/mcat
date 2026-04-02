//! `init` command handler for initializing repository metadata state.

use crate::errors::McatResult;
use crate::repos::{Repo, toml_repo::TomlDb};
use crate::services::scan_media;

use std::path::Path;

pub fn execute() -> McatResult<()> {
    let mut repo: TomlDb = Repo::init_empty();
    scan_media(&mut repo, Path::new("media/"), false)?;
    repo.persist()
}
