//! Repository abstractions and concrete repository module declarations.

use std::collections::BTreeSet;
use std::path::Path;

use crate::errors::McatResult;
use crate::models::TagAttributes;

pub mod toml_repo;

pub trait Repo {
    /// init an empty repo
    fn init_empty() -> Self
    where
        Self: Sized;

    /// insert a track to the repo
    fn insert_track(&mut self, file_hash: String, tag_attr: TagAttributes);

    /// remove a track from the repo,
    /// returns `true` if the track exists
    fn remove_track(&mut self, file_hash: &str) -> McatResult<()>;

    /// get all track hashes from the repo
    fn get_track_hashes(&self) -> BTreeSet<String>;

    /// read repo from a file
    fn from(file_path: impl AsRef<Path>) -> McatResult<Self>
    where
        Self: Sized;

    fn persist(&self) -> McatResult<()>;
}
