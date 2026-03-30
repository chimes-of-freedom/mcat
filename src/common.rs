use crate::McatError;

use std::fs::File;
use std::io::{self, Read, Seek};
use std::path::Path;

use blake3::Hasher;
use lofty::{file::TaggedFile, prelude::*, probe::Probe, tag::Tag};
use std::fs::OpenOptions;

fn get_tagged_file(file_path: impl AsRef<Path>) -> Result<TaggedFile, McatError> {
    Probe::open(file_path.as_ref())?.read().map_err(Into::into)
}

pub fn get_primary_tag(file_path: impl AsRef<Path>) -> Result<Tag, McatError> {
    let file_path = file_path.as_ref();

    if !file_path.is_file() {
        return Err(McatError::FileNotFound);
    }

    let tagged_file = get_tagged_file(file_path)?;

    match tagged_file.primary_tag() {
        Some(tag) => Ok(tag.clone()),
        None => match tagged_file.first_tag() {
            Some(tag) => Ok(tag.clone()),
            None => Err(McatError::TagNotFound),
        },
    }
}

pub fn get_file_hash(path: impl AsRef<Path>) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Hasher::new();
    let mut buf = [0u8; 8192];

    loop {
        let n = file.read(&mut buf)?;
        if n == 0 {
            break;
        }
        hasher.update(&buf[..n]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

/// strip tags from a music file
///
/// usually used for a stable file hash
pub fn strip_tags_from_file(path: impl AsRef<Path>) -> Result<(), McatError> {
    let tagged_file = get_tagged_file(&path)?;
    let tags = tagged_file.tags();

    if tags.is_empty() {
        return Ok(());
    }

    let mut file = OpenOptions::new().read(true).write(true).open(&path)?;

    // use `remove_from()` rather than
    // `remove_from_file()` with more efficiency
    for tag in tags {
        tag.tag_type().remove_from(&mut file)?;
        // `remove_from()` will fail to detect the type of the file
        // without seeking file from the start
        file.seek(io::SeekFrom::Start(0))?;
    }

    Ok(())
}

/// check if a file is supported by lofty
pub fn is_file_supported(path: impl AsRef<Path>) -> Result<bool, McatError> {
    match Probe::open(&path)?.guess_file_type() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}
