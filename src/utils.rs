use std::path::Path;
use lofty::{file::TaggedFile, prelude::*, probe::Probe, tag::Tag};

use crate::McatError;

fn get_tagged_file(file_path: &Path) -> Result<TaggedFile, McatError> {
    Probe::open(file_path)
        .map_err(|_| McatError::OpenFailed)?
        .read()
        .map_err(|_| McatError::ReadFailed)
}

pub fn get_primary_tag(file_path: &Path) -> Result<Tag, McatError> {
    if !file_path.is_file() {
        return Err(McatError::FileNotFound);
    }

    let tagged_file = get_tagged_file(file_path)?;

    match tagged_file.primary_tag() {
        Some(tag) => Ok(tag.clone()),
        None => match tagged_file.first_tag() {
            Some(tag) => Ok(tag.clone()),
            None => Err(McatError::TagNotFound),
        }
    }
}

pub fn display_tag(primary_tag: &Tag) {
    println!("--- Tag Info ---");
    println!(
        "Title: {}",
        primary_tag.title().as_deref().unwrap_or("None")
    );
    println!(
        "Artist: {}",
        primary_tag.artist().as_deref().unwrap_or("None")
    );
    println!(
        "Album: {}",
        primary_tag.album().as_deref().unwrap_or("None")
    );
    println!(
        "Genre: {}",
        primary_tag.genre().as_deref().unwrap_or("None")
    );
    println!(
        "Album Artist: {}",
        primary_tag
            .get_string(ItemKey::AlbumArtist)
            .unwrap_or("None"),
    );
}
