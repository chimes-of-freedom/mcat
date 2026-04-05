//! Repository abstractions and concrete repository module declarations.

use std::collections::BTreeSet;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::errors::McatResult;
use crate::models::TagAttributes;

pub mod toml_repo;

/// Repository abstraction. Currently implemented by [`toml_repo::TomlDb`].
pub trait Repo {
    /// Initializes an empty repository.
    fn init_empty() -> Self
    where
        Self: Sized;

    /// Inserts a track into the repository.
    fn insert_track(&mut self, file_hash: String, tag_attr: TagAttributes);

    /// Removes a track from the repository.
    ///
    /// # Errors
    ///
    /// Returns [`crate::errors::McatError::TrackNotFound`] if the track
    /// does not exist.
    fn remove_track(&mut self, file_hash: &str) -> McatResult<()>;

    /// Queries a track by its hash.
    fn query_track_by_hash(&self, file_hash: &str) -> Option<Entry>;

    /// Queries a track by its title.
    fn query_track_by_title(&self, title: &str) -> Option<Entry>;

    /// Returns all track hashes in the repository.
    fn get_track_hashes(&self) -> BTreeSet<String>;

    /// Returns tag attributes for all tracks.
    fn get_tag_attrs(&self) -> Vec<&TagAttributes>;

    /// Loads a repository from a file.
    ///
    /// # Errors
    ///
    /// Returns I/O or TOML deserialization related errors while
    /// opening the file or deserializing its content.
    fn from(file_path: impl AsRef<Path>) -> McatResult<Self>
    where
        Self: Sized;

    /// Persists the repository to its backing file.
    ///
    /// # Errors
    ///
    /// Returns I/O or serialization related errors while writing
    /// repository data to disk.
    fn persist(&mut self) -> McatResult<()>;
}

/// An [`Entry`] records a file's BLAKE3 hash and its metadata.
#[derive(Serialize, Deserialize, Clone)]
pub struct Entry {
    pub file_hash: String,

    pub tag_attr: TagAttributes,
}
