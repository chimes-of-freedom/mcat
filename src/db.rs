use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use blake3::Hasher;
use serde::{Deserialize, Serialize};

use crate::{
    McatError, TagAttributes,
    common::{self, get_primary_tag},
};

/// The databse of mcat.
#[derive(Serialize, Deserialize, Debug)]
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
    pub fn from_file(toml_path: impl AsRef<Path>) -> Result<Self, McatError> {
        let db_string = fs::read_to_string(toml_path)?;
        let db = toml::from_str(&db_string)?;

        Ok(db)
    }

    /// write to db file
    pub fn to_file(&self) -> Result<(), McatError> {
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

    /// scan media directory and init db
    pub fn scan(&mut self, media_dir: impl AsRef<Path>) -> Result<(), McatError> {
        let files = fs::read_dir(media_dir)?;

        for file in files {
            let file = file?;
            let file_type = file.file_type()?;
            let file_path = file.path();

            if file_type.is_file() && common::is_file_supported(&file_path)? {
                // NOTE: get the tag before stripping it from file!
                let tag = get_primary_tag(&file_path)?;
                let tag_attr = TagAttributes::from_tag(&tag);

                common::strip_tags_from_file(&file_path)?;
                let file_hash = common::get_file_hash(&file_path)?;

                self.insert_entry(Entry {
                    file_hash,
                    tag_attr,
                });
            }
        }

        Ok(())
    }

    /// insert an entry
    pub fn insert_entry(&mut self, entry: Entry) {
        let key = entry.file_hash.clone();
        self.entries.insert(key, entry);
        self.update_hash();
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

/// An `Entry` matches a single file.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    file_hash: String,

    tag_attr: TagAttributes,
}
