//! TOML-backed repository implementation and persistence utilities.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::errors::McatResult;
use crate::models::TagAttributes;
use crate::repos::Repository;

/// An `Entry` matches a single file.
#[derive(Serialize, Deserialize)]
pub struct Entry {
    pub file_hash: String,

    pub tag_attr: TagAttributes,
}

/// The databse of mcat.
#[derive(Serialize, Deserialize)]
pub struct Database {
    // total hash for files
    total_hash: String,

    // use file hash as key
    // `BTreeMap` ensures entries are sorted by key,
    // which makes the total hash deterministic
    entries: BTreeMap<String, Entry>,
}

impl Database {
    /// init a database
    pub fn new() -> Self {
        Database {
            total_hash: String::new(),
            entries: BTreeMap::new(),
        }
    }

    /// read from db file
    pub fn from_file(toml_path: impl AsRef<Path>) -> McatResult<Self> {
        let db_string = fs::read_to_string(toml_path)?;
        let db = toml::from_str(&db_string)?;

        Ok(db)
    }

    /// write to db file
    pub fn to_file(&self) -> McatResult<()> {
        let toml_path = PathBuf::from(".mcat/db.toml");
        let bak_path = PathBuf::from(".mcat/db.toml.bak");
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

    /// insert an entry
    pub fn insert_entry(&mut self, entry: Entry) {
        let key = entry.file_hash.clone();
        self.entries.insert(key, entry);
        self.update_hash();
    }

    /// remove an entry
    pub fn remove_entry(&mut self, key: &str) -> Option<Entry> {
        self.entries.remove(key)
    }

    /// update `total_hash` after changing `entries`
    fn update_hash(&mut self) {
        let mut hasher = Hasher::new();

        for (key, _) in self.entries.iter() {
            hasher.update(key.as_bytes());
        }

        self.total_hash = hasher.finalize().to_hex().to_string();
    }
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Repository for Database {
    fn init_empty() -> Self {
        Self::new()
    }

    fn insert_track(&mut self, file_hash: String, tag_attr: TagAttributes) {
        self.insert_entry(Entry {
            file_hash,
            tag_attr,
        });
    }

    fn remove_track(&mut self, file_hash: &str) -> bool {
        self.remove_entry(file_hash).is_some()
    }

    fn get_track_hashes(&self) -> BTreeSet<String> {
        self.entries.keys().cloned().collect()
    }

    fn from(file_path: impl AsRef<Path>) -> McatResult<Self> {
        Self::from_file(file_path)
    }

    fn persist(&self) -> McatResult<()> {
        self.to_file()
    }
}
