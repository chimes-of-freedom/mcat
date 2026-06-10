//! Repositories for different kinds of entities, such as tracks and playlists.
//! Types reflected to repositories are re-exported.

mod track_repo;

use std::path::{Path, PathBuf};

use lofty::{picture::PictureType, prelude::*, tag::Tag};
use rusqlite::Row;
pub use track_repo::TrackRepo;

use crate::models::{
    NewImage, NewLyrics, NewTrack, NewTrackFile, NewTrackMetadata, TrackMetadataCore, TrackRow,
};

impl TrackRow {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Self {
            id: row.get("id")?,
            core: TrackMetadataCore {
                title: row.get("title")?,
                artist: row.get("artist")?,
                album: row.get("album")?,
                album_artist: row.get("album_artist")?,
                recording_date: row.get("recording_date")?,
                release_date: row.get("release_date")?,
                track_number: row.get("track_number")?,
                disc_number: row.get("disc_number")?,
                genre: row.get("genre")?,
                composer: row.get("composer")?,
                lyricist: row.get("lyricist")?,
            },
            lyrics_id: row.get("lyrics_id")?,
            front_cover_id: row.get("front_cover_id")?,
            file_id: row.get("file_id")?,
        })
    }
}

impl From<&Tag> for NewTrackMetadata {
    fn from(tag: &Tag) -> Self {
        Self {
            core: TrackMetadataCore {
                title: tag.title().unwrap_or_default().to_string(),
                artist: tag.artist().map(String::from),
                album: tag.album().map(String::from),
                album_artist: tag.get_string(ItemKey::AlbumArtist).map(String::from),
                recording_date: tag.get_string(ItemKey::RecordingDate).map(String::from),
                release_date: tag.get_string(ItemKey::ReleaseDate).map(String::from),
                track_number: tag
                    .get_string(ItemKey::TrackNumber)
                    .and_then(|s| s.parse().ok()),
                disc_number: tag
                    .get_string(ItemKey::DiscNumber)
                    .and_then(|s| s.parse().ok()),
                genre: tag.genre().map(String::from),
                composer: tag.get_string(ItemKey::Composer).map(String::from),
                lyricist: tag.get_string(ItemKey::Lyricist).map(String::from),
            },
            lyrics: tag
                .get_binary(ItemKey::Lyrics, true)
                .map(|b| NewLyrics { data: b.to_vec() }),
            front_cover: tag
                .get_picture_type(PictureType::CoverFront)
                .map(|p| NewImage {
                    data: p.data().to_vec(),
                }),
        }
    }
}

impl NewTrack {
    pub fn from_tag(tag: &Tag, path: impl AsRef<Path>) -> Self {
        Self {
            metadata: NewTrackMetadata::from(tag),
            file: NewTrackFile::from(PathBuf::from(path.as_ref())),
        }
    }
}
