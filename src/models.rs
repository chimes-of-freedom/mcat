//! Core domain models used across commands and services.

use std::collections::BTreeSet;

use lofty::tag::Tag;
use lofty::{picture::PictureType, prelude::*};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::{config, errors::McatResult, repos::Repo};

/// Metadata fields extracted from a media file tag.
#[derive(Serialize, Deserialize, Clone, Tabled)]
#[tabled(display(Option, "tabled::derive::display::option", ""))]
pub struct TagAttributes {
    /// Track title.
    pub title: Option<String>,

    /// Track artist.
    pub artist: Option<String>,

    /// Album title.
    pub album: Option<String>,

    /// Album artist.
    pub album_artist: Option<String>,

    /// Genre.
    pub genre: Option<String>,

    /// Front Cover.
    #[tabled(skip)]
    pub front_cover: Option<Image>,
}

impl TagAttributes {
    /// Returns `true` when all tag fields are absent.
    pub fn is_empty(&self) -> bool {
        matches!(
            self,
            TagAttributes {
                title: None,
                artist: None,
                album: None,
                album_artist: None,
                genre: None,
                front_cover: None,
            }
        )
    }

    /// Builds [`TagAttributes`] from a [`Tag`].
    pub fn from_tag(tag: &Tag) -> TagAttributes {
        let front_cover = tag
            .get_picture_type(PictureType::CoverFront)
            .map(|cover| Image {
                mime_type: cover.mime_type().map(|m| m.to_string()),
                description: cover.description().map(|s| s.to_string()),
                data: ImageData::Inline(cover.data().to_vec()),
            });

        TagAttributes {
            title: tag.title().as_deref().map(str::to_string),
            artist: tag.artist().as_deref().map(str::to_string),
            album: tag.album().as_deref().map(str::to_string),
            album_artist: tag.get_string(ItemKey::AlbumArtist).map(str::to_string),
            genre: tag.genre().as_deref().map(str::to_string),
            front_cover,
        }
    }
}

/// Image fields extracted from a media file tag.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub mime_type: Option<String>,
    pub description: Option<String>,
    #[serde(flatten)]
    pub data: ImageData,
}

/// Enum for image payload. [`ImageData::Inline`] represents the full image while
/// [`ImageData::Linked`] represents a link to file on disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ImageData {
    /// Inline data.
    #[serde(skip)]
    Inline(Vec<u8>),

    /// External file name.
    Linked { file_name: String },
}

impl Image {
    pub fn into_inline(self, data: Vec<u8>) -> Self {
        Self {
            data: ImageData::Inline(data),
            ..self
        }
    }

    pub fn into_linked(self, file_name: String) -> Self {
        Self {
            data: ImageData::Linked { file_name },
            ..self
        }
    }

    /// Converts [`Image::data`] from [`ImageData::Inline`] to
    /// [`ImageData::Linked`] and writes data back to disk. Does nothing when
    /// [`Image::data`] is already a [`ImageData::Linked`].
    ///
    /// # Errors
    ///
    /// Returns I/O related errors when writing data back to disk.
    pub fn linked_and_to_disk(self, file_hash: &str) -> McatResult<Image> {
        match &self.data {
            ImageData::Inline(data) => {
                // Extract extension from mime type (e.g., "image/jpeg" -> "jpeg")
                let ext = self
                    .mime_type
                    .as_deref()
                    .and_then(|m| m.split('/').next_back())
                    .unwrap_or("bin");

                let file_name = format!("{}.{}", file_hash, ext);
                let mut image_path = config::cover_dir_path();
                image_path.push(&file_name);

                // Write image data back to disk
                std::fs::write(&image_path, data)?;

                Ok(self.into_linked(file_name))
            }
            _ => Ok(self),
        }
    }
}

/// Filter for querying track metadata.
/// Fields logically ORed within the same field and ANDed across different
/// fields. For example, `artist: [A, B], genre: [X, Y]` matches tracks with
/// `(artist == A OR artist == B) AND (genre == X OR genre == Y)`.
#[derive(Serialize, Deserialize)]
pub struct TrackFilter {
    pub titles: BTreeSet<String>,
    pub artists: BTreeSet<String>,
    pub albums: BTreeSet<String>,
    pub album_artists: BTreeSet<String>,
    pub genres: BTreeSet<String>,

    pub hashes: BTreeSet<String>,
}

impl TrackFilter {
    pub fn new(
        titles: Vec<String>,
        artists: Vec<String>,
        albums: Vec<String>,
        album_artists: Vec<String>,
        genres: Vec<String>,
        hashes: Vec<String>,
    ) -> Self {
        TrackFilter {
            titles: titles.into_iter().collect(),
            artists: artists.into_iter().collect(),
            albums: albums.into_iter().collect(),
            album_artists: album_artists.into_iter().collect(),
            genres: genres.into_iter().collect(),
            hashes: hashes.into_iter().collect(),
        }
    }

    /// Applies the filter to the repository, returning hashes of matching
    /// tracks.
    pub fn apply<T: Repo>(self, repo: &T) -> Vec<String> {
        let matches_opt = |filters: &BTreeSet<String>, value: Option<&String>| {
            filters.is_empty() || value.is_some_and(|v| filters.contains(v))
        };

        repo.get_track_hashes()
            .into_iter()
            .filter(|hash| self.hashes.is_empty() || self.hashes.contains(hash))
            .filter(|hash| {
                let Some(entry) = repo.query_track_by_hash(hash) else {
                    return false;
                };

                matches_opt(&self.titles, entry.tag_attr.title.as_ref())
                    && matches_opt(&self.artists, entry.tag_attr.artist.as_ref())
                    && matches_opt(&self.albums, entry.tag_attr.album.as_ref())
                    && matches_opt(&self.album_artists, entry.tag_attr.album_artist.as_ref())
                    && matches_opt(&self.genres, entry.tag_attr.genre.as_ref())
            })
            .collect()
    }
}

/// Result of consistency checks between media files and repository records.
#[derive(Serialize, Deserialize)]
pub struct CheckResult {
    /// Hashes found in media files but not tracked in the repository.
    pub not_tracked: Option<BTreeSet<String>>,

    /// Hashes tracked in the repository but missing from the media directory.
    pub not_exists: Option<BTreeSet<String>>,
}
