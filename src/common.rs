use crate::McatError;

use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

use blake3::Hasher;
use lofty::{file::TaggedFile, prelude::*, probe::Probe, tag::Tag};

fn get_tagged_file<P: AsRef<Path>>(file_path: P) -> Result<TaggedFile, McatError> {
    let file_path = file_path.as_ref();

    Probe::open(file_path)
        .map_err(|_| McatError::OpenFailed)?
        .read()
        .map_err(|_| McatError::ReadFailed)
}

pub fn get_primary_tag<P: AsRef<Path>>(file_path: P) -> Result<Tag, McatError> {
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

pub fn get_file_hash<P: AsRef<Path>>(path: P) -> io::Result<String> {
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
