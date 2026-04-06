//! `display` command handler for querying and showing track metadata.

use mcat::{
    config,
    errors::McatResult,
    output::display_tag_attrs,
    repos::{Repo, toml_repo::TomlDb},
};

/// Executes the `display` command.
///
/// # Errors
///
/// Returns repository loading errors.
pub fn execute() -> McatResult<()> {
    let db: TomlDb = Repo::from(config::repo_file_path())?;

    let tag_attrs = db.get_tag_attrs();

    display_tag_attrs(&tag_attrs);

    Ok(())
}
