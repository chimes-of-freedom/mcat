//! TOML-backed repository implementation and persistence utilities.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::repos::Repo;
use crate::{config, models::*};
use crate::{
    errors::{McatError, McatResult},
    repos::Entry,
};

/// TOML-backed repository database used by mcat.
#[derive(Serialize, Deserialize)]
pub struct TomlDb {
    // Aggregate hash computed from sorted entry keys.
    total_hash: String,

    // Uses file hash as key.
    // `BTreeMap` keeps keys sorted, making the aggregate hash deterministic.
    entries: BTreeMap<String, Entry>,
}

impl TomlDb {
    /// Creates an empty database.
    pub fn new() -> Self {
        TomlDb {
            total_hash: String::new(),
            entries: BTreeMap::new(),
        }
    }

    /// Loads a database from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns I/O errors when reading the file and TOML deserialization
    /// errors when parsing content.
    pub fn from_file(toml_path: impl AsRef<Path>) -> McatResult<Self> {
        let db_string = fs::read_to_string(toml_path)?;
        let db = toml::from_str(&db_string)?;

        Ok(db)
    }

    /// Writes the database to [`config::repo_file_path`] with a temporary backup.
    ///
    /// # Errors
    ///
    /// Returns I/O and TOML serialization errors raised during backup, file
    /// creation, writing, or cleanup.
    pub fn to_file(&self) -> McatResult<()> {
        let toml_path = config::repo_file_path();
        let bak_path = config::repo_backup_file_path();
        let exists = toml_path.try_exists()?;

        // if the original database exists, try to backup it
        // otherwise just create `.mcat/` directory if not exists
        if exists {
            fs::copy(&toml_path, &bak_path)?;
        } else {
            let parent_path = PathBuf::from(".mcat/");
            fs::create_dir_all(&parent_path)?;
        }

        // write to new db file
        let db_string = toml::to_string(self)?;
        fs::write(&toml_path, &db_string)?;

        // if no error occurs, remove the backup if exists
        if exists {
            fs::remove_file(&bak_path)?;
        }

        Ok(())
    }

    /// Inserts an entry into the repository.
    pub fn insert_entry(&mut self, entry: Entry) {
        let key = entry.file_hash.clone();
        self.entries.insert(key, entry);
    }

    /// Removes an entry by key.
    pub fn remove_entry(&mut self, key: &str) -> McatResult<Option<Entry>> {
        let Some(entry) = self.entries.remove(key) else {
            return Ok(None);
        };

        let Some(image) = &entry.tag_attr.front_cover else {
            return Ok(Some(entry));
        };

        let ImageData::Linked { file_name } = &image.data else {
            return Ok(Some(entry));
        };

        fs::remove_file(file_name)?;

        Ok(Some(entry))
    }

    /// Recomputes `total_hash` from current entry keys.
    fn update_hash(&mut self) {
        let mut hasher = Hasher::new();

        for (key, _) in self.entries.iter() {
            hasher.update(key.as_bytes());
        }

        self.total_hash = hasher.finalize().to_hex().to_string();
    }
}

impl Default for TomlDb {
    fn default() -> Self {
        Self::new()
    }
}

impl Repo for TomlDb {
    fn init_empty() -> Self {
        Self::new()
    }

    fn insert_track(&mut self, file_hash: String, tag_attr: TagAttributes) {
        self.insert_entry(Entry {
            file_hash,
            tag_attr,
        });
    }

    fn remove_track(&mut self, file_hash: &str) -> McatResult<()> {
        if self.remove_entry(file_hash)?.is_some() {
            Ok(())
        } else {
            Err(McatError::TrackNotFound)
        }
    }

    fn query_track_by_hash(&self, file_hash: &str) -> Option<Entry> {
        self.entries.get(file_hash).cloned()
    }

    fn query_track_by_title(&self, title: &str) -> Option<Entry> {
        self.entries
            .values()
            .find(|entry| entry.tag_attr.title.as_deref() == Some(title))
            .cloned()
    }

    fn get_track_hashes(&self) -> BTreeSet<String> {
        self.entries.keys().cloned().collect()
    }

    fn get_tag_attrs(&self) -> Vec<&TagAttributes> {
        self.entries.values().map(|entry| &entry.tag_attr).collect()
    }

    fn from(file_path: impl AsRef<Path>) -> McatResult<Self> {
        Self::from_file(file_path)
    }

    fn persist(&mut self) -> McatResult<()> {
        self.update_hash();
        self.to_file()
    }
}
