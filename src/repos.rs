//! Repository abstractions and concrete repository module declarations.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

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

    /// query a track using its hash
    fn query_track_by_hash(&self, file_hash: &str) -> Option<Entry>;

    /// query a track using its title
    fn query_track_by_title(&self, title: &str) -> Option<Entry>;

    /// get all track hashes from the repo
    fn get_track_hashes(&self) -> BTreeSet<String>;

    /// read repo from a file
    fn from(file_path: impl AsRef<Path>) -> McatResult<Self>
    where
        Self: Sized;

    /// save db struct to file
    fn persist(&mut self) -> McatResult<()>;
}

/// An `Entry` matches a single file.
#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub file_hash: String,

    pub tag_attr: TagAttributes,
}
