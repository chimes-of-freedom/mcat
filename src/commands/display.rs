//! `display` command handler for querying and showing track metadata.

use mcat::{
    errors::McatResult,
    output::display_tag_attrs,
    repos::{Repo, toml_repo::TomlDb},
};

pub fn execute() -> McatResult<()> {
    let db: TomlDb = Repo::from(".mcat/db.toml")?;

    let tag_attrs = db.get_tag_attrs();

    display_tag_attrs(&tag_attrs);

    Ok(())
}
