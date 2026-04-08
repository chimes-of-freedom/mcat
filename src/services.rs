//! Reusable service-layer utilities for media and metadata workflows.

use crate::config;
use crate::errors::{McatError, McatResult};
use crate::models::TagAttributes;
use crate::repos::Repo;

use std::fs::{self, File};
use std::io::{self, Cursor, Read, Seek};
use std::path::Path;

use blake3::Hasher;
use lofty::config::WriteOptions;
use lofty::{file::TaggedFile, prelude::*, probe::Probe, tag::Tag};
use std::fs::OpenOptions;

/// Loads a `TaggedFile` from the given path.
fn get_tagged_file(file_path: impl AsRef<Path>) -> McatResult<TaggedFile> {
    Probe::open(file_path.as_ref())?.read().map_err(Into::into)
}

/// Returns the primary tag from a file, or its first tag
/// if a primary tag doesn't exist.
///
/// # Errors
///
/// - Returns [`McatError::FileNotFound`] if `file_path` does not exist or is
///   not a regular file.
/// - Returns [`McatError::TagNotFound`] if the file does not contain any tag.
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

/// Calculates the BLAKE3 hash of a file.
///
/// # Errors
///
/// Returns [`McatError::Io`] raised while opening or reading the file.
pub fn get_hash_from_file(path: impl AsRef<Path>) -> McatResult<String> {
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

/// Calculates the BLAKE3 hash of a byte slice.
pub fn get_hash_from_vec(data: &[u8]) -> String {
    let mut hasher = Hasher::new();

    hasher.update(data);

    hasher.finalize().to_hex().to_string()
}

/// Removes tags from a file. When `saved` is `true`, writes the changes back
/// to the file; otherwise, returns the stripped file data.
///
/// This is commonly used to compute a stable file hash.
///
/// # Errors
///
/// - Returns [`McatError::FileNotFound`] when `path` does not exist or is not a
///   regular file.
/// - Returns [`McatError::TagNotFound`] when the file does not contain any
///   readable tag.
/// - Returns a file-system or metadata error when the file cannot be opened,
///   read, seeked, stripped, or written back.
pub fn strip_tags_from_file(path: impl AsRef<Path>, saved: bool) -> McatResult<Option<Vec<u8>>> {
    if saved {
        let tagged_file = get_tagged_file(&path)?;
        let tags = tagged_file.tags();

        if tags.is_empty() {
            return Ok(None);
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

        Ok(None)
    } else {
        let mut buffer = Vec::new();
        File::open(&path)?.read_to_end(&mut buffer)?;
        let mut cursor = Cursor::new(buffer);
        let mut tagged_file = Probe::new(&mut cursor).guess_file_type()?.read()?;

        tagged_file.clear();

        tagged_file.save_to(&mut cursor, WriteOptions::default())?;

        Ok(Some(cursor.into_inner()))
    }
}

/// Returns whether `lofty` can recognize the file format.
///
/// # Errors
///
/// Probe failures are treated as `Ok(false)`, so this function returns
/// an error only if [`Probe::open`] fails.
pub fn is_file_supported(path: impl AsRef<Path>) -> McatResult<bool> {
    match Probe::open(&path)?.guess_file_type() {
        Ok(_) => Ok(true),
        Err(_) => Ok(false),
    }
}

/// Returns inferred mime type string of given file.
/// 
/// # Errors
/// 
/// Returns an error if failed to read the path.
pub fn infer_mime_type(path: impl AsRef<Path>) -> McatResult<&'static str> {
    Ok(infer::get_from_path(path)?
        .map(|t| t.mime_type())
        .unwrap_or("application/octet-stream"))
}

/// Returns whether `s` is a valid BLAKE3 hash.
pub fn is_valid_blake3_hex(s: &str) -> bool {
    blake3::Hash::from_hex(s).is_ok()
}

/// Scans a media directory and inserts supported files into the repository.
///
/// # Errors
///
/// - Returns a file-system error when `media_dir` cannot be read as a
///   directory.
/// - Returns [`McatError::TagNotFound`] when a supported media file does not
///   contain any readable tag.
/// - Returns a file-system or metadata error when reading directory entries,
///   probing files, stripping tags, or hashing file contents fails.
pub fn scan_media(
    repo: &mut impl Repo,
    media_dir: impl AsRef<Path>,
    saved: bool,
) -> McatResult<()> {
    let files = fs::read_dir(media_dir)?;

    // Create cover directory if not exists
    let cover_dir = config::cover_dir_path();
    if cover_dir.exists() && !cover_dir.is_dir() {
        fs::remove_file(&cover_dir)?;
    }
    if !cover_dir.exists() {
        fs::create_dir_all(&cover_dir)?;
    }

    for file in files {
        let file = file?;
        let file_type = file.file_type()?;
        let file_path = file.path();

        if file_type.is_file() && is_file_supported(&file_path)? {
            // NOTE: get the tag before stripping it from file!
            let tag = get_primary_tag(&file_path)?;
            let mut tag_attr = TagAttributes::from_tag(&tag);

            let file_hash = if saved {
                strip_tags_from_file(&file_path, saved)?;
                get_hash_from_file(&file_path)?
            } else {
                let stripped_data = strip_tags_from_file(&file_path, saved)?.unwrap();
                get_hash_from_vec(&stripped_data)
            };

            // NOTE: parse `ImageData::Inline` to `ImageData::Linked` before inserting
            // a track!
            if let Some(image) = tag_attr.front_cover {
                tag_attr.front_cover = Some(image.linked_and_to_disk(&file_hash)?);
            }

            repo.insert_track(file_hash, tag_attr);
        }
    }

    Ok(())
}
