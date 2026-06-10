//! Types related to tracks in the track repository.

use std::path::PathBuf;

use serde::Serialize;

pub struct Track {
    pub id: i64,
    pub metadata: TrackMetadata,
    pub file: TrackFile,
}

impl Track {
    pub fn from_new(
        new_track: NewTrack,
        id: i64,
        lyrics: Option<Lyrics>,
        front_cover: Option<Image>,
        track_file: TrackFile,
    ) -> Self {
        Self {
            id,
            metadata: TrackMetadata {
                core: new_track.metadata.core,
                lyrics,
                front_cover,
            },
            file: track_file,
        }
    }
}

pub struct NewTrack {
    pub metadata: NewTrackMetadata,
    pub file: NewTrackFile,
}

#[derive(Serialize)]
pub struct TrackRow {
    pub id: i64,

    #[serde(flatten)]
    pub core: TrackMetadataCore,

    pub lyrics_id: Option<i64>,
    pub front_cover_id: Option<i64>,
    pub file_id: Option<i64>,
}

pub struct TrackMetadata {
    pub core: TrackMetadataCore,

    /// Lyrics.
    pub lyrics: Option<Lyrics>,

    /// Front Cover.
    pub front_cover: Option<Image>,
}

#[derive(Serialize, Clone, Hash, PartialEq, Eq)]
pub struct TrackMetadataCore {
    /// Track title.
    pub title: String,
    /// Track artist.
    pub artist: Option<String>,
    /// Album title.
    pub album: Option<String>,
    /// Album artist.
    pub album_artist: Option<String>,
    /// Recording date.
    pub recording_date: Option<String>,
    /// Release date.
    pub release_date: Option<String>,
    /// Track number.
    pub track_number: Option<i64>,
    /// Disc number.
    pub disc_number: Option<i64>,
    /// Genre.
    pub genre: Option<String>,
    /// Composer.
    pub composer: Option<String>,
    /// Lyricist.
    pub lyricist: Option<String>,
}

pub struct NewTrackMetadata {
    pub core: TrackMetadataCore,
    pub lyrics: Option<NewLyrics>,
    pub front_cover: Option<NewImage>,
}

pub struct Lyrics {
    pub id: i64,
    pub hash: Vec<u8>,
    pub data: Option<Vec<u8>>,
}

impl Lyrics {
    pub fn from_new(new_lyrics: NewLyrics, id: i64, hash: Vec<u8>) -> Self {
        Self {
            id,
            hash,
            data: Some(new_lyrics.data),
        }
    }
}

#[derive(Clone)]
pub struct NewLyrics {
    pub data: Vec<u8>,
}

pub struct Image {
    pub id: i64,
    pub hash: Vec<u8>,
    pub data: Option<Vec<u8>>,
}

impl Image {
    pub fn from_new(new_image: NewImage, id: i64, hash: Vec<u8>) -> Self {
        Self {
            id,
            hash,
            data: Some(new_image.data),
        }
    }
}

#[derive(Clone)]
pub struct NewImage {
    pub data: Vec<u8>,
}

pub struct TrackFile {
    pub id: i64,
    pub name: String,
    pub hash: Vec<u8>,
}

pub struct NewTrackFile {
    pub path: PathBuf,
}

impl From<PathBuf> for NewTrackFile {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}
