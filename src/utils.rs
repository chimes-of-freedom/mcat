use std::path::Path;
use lofty::{file::TaggedFile, prelude::*, probe::Probe, tag::Tag, config::WriteOptions};

use crate::{McatError, TagAttributes};

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

pub fn edit_tag<P: AsRef<Path>>(output_path: P, tag: &mut Tag, tag_attrs: TagAttributes) -> Result<(), McatError> {
    if tag_attrs.is_empty() {
        return Err(McatError::AttrEmpty);
    }

    if let Some(title) = tag_attrs.title {
        tag.set_title(title);
    }
    if let Some(artist) = tag_attrs.artist {
        tag.set_artist(artist);
    }
    if let Some(album) = tag_attrs.album {
        tag.set_album(album);
    }
    if let Some(genre) = tag_attrs.genre {
        tag.set_genre(genre);
    }

    tag.save_to_path(output_path.as_ref(), WriteOptions::default())
        .map_err(|_| McatError::WriteFailed)?;

    Ok(())
}
