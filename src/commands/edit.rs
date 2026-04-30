//! `edit` command handler for updating track metadata records.

use std::fs;

use crate::{
    cli::EditArgs,
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
/// Returns an error if:
///
/// - Loading or persisting the repository fails.
/// - The requested track cannot be found.
/// - Front-cover file checks or file operations fail.
/// - Inferring the front-cover MIME type fails.
pub fn execute(track: String, edit: EditArgs) -> McatResult<()> {
    let mut repo = TomlDb::try_from(config::repo_file_path())?;

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

    // update metadata which doesn't need to be stored to files
    if edit.title.is_some() {
        tag_attr.title = edit.title;
    }
    if edit.artist.is_some() {
        tag_attr.artist = edit.artist;
    }
    if edit.album.is_some() {
        tag_attr.album = edit.album;
    }
    if edit.album_artist.is_some() {
        tag_attr.album_artist = edit.album_artist;
    }
    if edit.date.is_some() {
        tag_attr.date = edit.date;
    }
    if edit.track_number.is_some() {
        tag_attr.track_number = edit.track_number;
    }
    if edit.disc_number.is_some() {
        tag_attr.disc_number = edit.disc_number;
    }
    if edit.genre.is_some() {
        tag_attr.genre = edit.genre;
    }
    if edit.composer.is_some() {
        tag_attr.composer = edit.composer;
    }
    if edit.lyricist.is_some() {
        tag_attr.lyricist = edit.lyricist;
    }

    // update front cover
    if let Some(new_front_cover) = edit.front_cover {
        // ensure new image exists
        if new_front_cover.try_exists()? && new_front_cover.is_file() {
            // generate path to new image
            let new_mime_type = infer_mime_type(&new_front_cover)?;
            let new_ext = new_mime_type.split('/').next_back().unwrap_or("bin");
            let new_image_name = format!("{}.{}", &file_hash, new_ext);
            let new_image_path = config::cover_dir_path().join(&new_image_name);

            // copy new image file to images folder
            fs::copy(&new_front_cover, &new_image_path)?;

            // remove old image file if not covered by new image file
            if let Some(old_image) = &tag_attr.front_cover
                && let ImageData::Linked {
                    file_name: old_image_name,
                } = &old_image.data
                && old_image_name != &new_image_name
            {
                let mut old_image_path = config::cover_dir_path();
                old_image_path.push(old_image_name);
                fs::remove_file(&old_image_path)?;
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
