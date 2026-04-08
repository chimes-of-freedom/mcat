//! `edit` command handler for updating track metadata records.

use std::{
    fs,
    path::{Path, PathBuf},
};

use mcat::{
    config,
    errors::{McatError, McatResult},
    models::{Image, ImageData},
    repos::{Repo, toml_repo::TomlDb},
    services::{infer_mime_type, is_valid_blake3_hex},
};

/// Executes the `edit` command.
///
/// # Errors
///
/// This command is currently unimplemented and always panics via `todo!`.
pub fn execute(
    track: String,
    title: Option<String>,
    artist: Option<String>,
    album: Option<String>,
    album_artist: Option<String>,
    genre: Option<String>,
    front_cover: Option<impl AsRef<Path>>,
) -> McatResult<()> {
    let mut repo: TomlDb = Repo::from(config::repo_file_path())?;

    let entry = if is_valid_blake3_hex(&track) {
        repo.query_track_by_hash(&track)
    } else {
        repo.query_track_by_title(&track)
    };
    let Some(mut entry) = entry else {
        return Err(McatError::TrackNotFound);
    };

    let tag_attr = &mut entry.tag_attr;
    let file_hash = entry.file_hash.clone();

    // update metadata except front cover
    if let Some(title) = title {
        tag_attr.title = Some(title);
    }
    if let Some(artist) = artist {
        tag_attr.artist = Some(artist);
    }
    if let Some(album) = album {
        tag_attr.album = Some(album);
    }
    if let Some(album_artist) = album_artist {
        tag_attr.album_artist = Some(album_artist);
    }
    if let Some(genre) = genre {
        tag_attr.genre = Some(genre);
    }

    // update front cover
    if let Some(new_front_cover) = front_cover {
        let new_front_cover_path = PathBuf::from(new_front_cover.as_ref());

        // ensure new image exists
        if new_front_cover_path.try_exists()? && new_front_cover_path.is_file() {
            // generate path to new image
            let new_mime_type = infer_mime_type(&new_front_cover_path)?;
            let new_ext = new_mime_type.split('/').next_back().unwrap_or("bin");
            let new_image_name = format!("{}.{}", &file_hash, new_ext);
            let mut new_image_path = config::cover_dir_path();
            new_image_path.push(&new_image_name);

            // copy new image file to images folder
            fs::copy(&new_front_cover_path, &new_image_path)?;

            // remove old image file if not not covered by new image file
            if let Some(image) = &tag_attr.front_cover
                && let ImageData::Linked { file_name } = &image.data
                && file_name != &new_image_name
            {
                let mut old_file_path = config::cover_dir_path();
                old_file_path.push(file_name);
                fs::remove_file(&old_file_path)?;
            }

            // update `tag_attr.front_cover`
            tag_attr.front_cover = Some(Image {
                mime_type: Some(new_mime_type.to_string()),
                description: None,
                data: ImageData::Linked {
                    file_name: new_image_name,
                },
            });
        }
    }

    repo.insert_track(file_hash, entry.tag_attr);

    repo.persist()
}
