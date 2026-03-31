//! Reusable service-layer operations for media and metadata workflows.

use crate::errors::{McatError, McatResult};
use crate::models::TagAttributes;
use crate::repos::Repository;

use std::fs::{self, File};
use std::io::{self, Read, Seek};
use std::path::Path;

use blake3::Hasher;
use lofty::{file::TaggedFile, prelude::*, probe::Probe, tag::Tag};
use std::fs::OpenOptions;

fn get_tagged_file(file_path: impl AsRef<Path>) -> McatResult<TaggedFile> {
    Probe::open(file_path.as_ref())?.read().map_err(Into::into)
}

pub fn get_primary_tag(file_path: impl AsRef<Path>) -> McatResult<Tag> {
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
pub fn strip_tags_from_file(path: impl AsRef<Path>) -> McatResult<()> {
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
pub fn is_file_supported(path: impl AsRef<Path>) -> McatResult<bool> {
    match Probe::open(&path)?.guess_file_type() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// scan media directory and init db
pub fn scan_media(repo: &mut impl Repository, media_dir: impl AsRef<Path>) -> McatResult<()> {
    let files = fs::read_dir(media_dir)?;

    for file in files {
        let file = file?;
        let file_type = file.file_type()?;
        let file_path = file.path();

        if file_type.is_file() && is_file_supported(&file_path)? {
            // NOTE: get the tag before stripping it from file!
            let tag = get_primary_tag(&file_path)?;
            let tag_attr = TagAttributes::from_tag(&tag);

            strip_tags_from_file(&file_path)?;
            let file_hash = get_file_hash(&file_path)?;

            repo.insert_track(file_hash, tag_attr);
        }
    }

    Ok(())
}
