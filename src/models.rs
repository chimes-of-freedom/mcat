//! Core domain models used across commands and services.

use std::collections::BTreeSet;

use lofty::prelude::*;
use lofty::tag::Tag;
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
            }
        )
    }

    /// Builds [`TagAttributes`] from a [`Tag`].
    pub fn from_tag(tag: &Tag) -> TagAttributes {
        TagAttributes {
            title: tag.title().as_deref().map(str::to_string),
            artist: tag.artist().as_deref().map(str::to_string),
            album: tag.album().as_deref().map(str::to_string),
            album_artist: tag.get_string(ItemKey::AlbumArtist).map(str::to_string),
            genre: tag.genre().as_deref().map(str::to_string),
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
