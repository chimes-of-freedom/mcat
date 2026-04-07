//! Core domain models used across commands and services.

use std::collections::BTreeSet;

use lofty::tag::Tag;
use lofty::{picture::PictureType, prelude::*};
use serde::{Deserialize, Serialize};
use tabled::Tabled;

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
}

/// Result of consistency checks between media files and repository records.
#[derive(Serialize, Deserialize)]
pub struct CheckResult {
    /// Hashes found in media files but not tracked in the repository.
    pub not_tracked: Option<BTreeSet<String>>,

    /// Hashes tracked in the repository but missing from the media directory.
    pub not_exists: Option<BTreeSet<String>>,
}
