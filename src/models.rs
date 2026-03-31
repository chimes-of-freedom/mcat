//! Core domain models used across commands and services.

use lofty::prelude::*;
use lofty::tag::Tag;
use serde::{Deserialize, Serialize};

// should sync with members in `Edit`
#[derive(Serialize, Deserialize, Debug)]
pub struct TagAttributes {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub genre: Option<String>,
}

impl TagAttributes {
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

    /// parse `Tag` to `TagAttributes`
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
