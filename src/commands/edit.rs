//! `edit` command handler for updating track metadata records.

use std::path::PathBuf;

use mcat::errors::McatResult;

pub fn execute(
    _src: PathBuf,
    _title: Option<String>,
    _artist: Option<String>,
    _album: Option<String>,
    _album_artist: Option<String>,
    _genre: Option<String>,
    _dst: Option<PathBuf>,
) -> McatResult<()> {
    todo!("edit command is not implemented yet")
}
