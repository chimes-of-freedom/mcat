//! Core domain models used across commands and services.

use std::collections::BTreeSet;
use std::fs;
use std::str::FromStr;

use chrono::NaiveDate;
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

    /// Recording / Release date.
    pub date: Option<NaiveDate>,

    /// Track number.
    pub track_number: Option<usize>,

    /// Disc number.
    pub disc_number: Option<usize>,

    /// Genre.
    pub genre: Option<String>,

    /// Composer.
    pub composer: Option<String>,

    /// Lyricist.
    pub lyricist: Option<String>,

    /// Lyrics.
    #[tabled(skip)]
    pub lyrics: Option<Lyrics>,

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
                date: None,
                track_number: None,
                disc_number: None,
                genre: None,
                composer: None,
                lyricist: None,
                lyrics: None,
                front_cover: None,
            }
        )
    }
}

impl From<Tag> for TagAttributes {
    fn from(tag: Tag) -> TagAttributes {
        let lyrics = tag.get_string(ItemKey::Lyrics).map(|s| Lyrics {
            file_name: "".to_string(),
            data: s.to_string(),
        });
        let front_cover = tag
            .get_picture_type(PictureType::CoverFront)
            .map(|cover| Image {
                mime_type: cover.mime_type().map(|m| m.to_string()),
                description: cover.description().map(|s| s.to_string()),
                file_name: String::new(),
                data: cover.data().to_vec(),
            });

        TagAttributes {
            title: tag.title().as_deref().map(str::to_string),

            artist: tag.artist().as_deref().map(str::to_string),

            album: tag.album().as_deref().map(str::to_string),

            album_artist: tag.get_string(ItemKey::AlbumArtist).map(str::to_string),

            date: tag
                .get_string(ItemKey::RecordingDate)
                .and_then(|s| NaiveDate::from_str(s).ok()),

            track_number: tag
                .get_string(ItemKey::TrackNumber)
                .and_then(|s| s.parse().ok()),

            disc_number: tag
                .get_string(ItemKey::DiscNumber)
                .and_then(|s| s.parse().ok()),

            genre: tag.genre().as_deref().map(str::to_string),

            composer: tag.get_string(ItemKey::Composer).map(str::to_string),

            lyricist: tag.get_string(ItemKey::Lyricist).map(str::to_string),

            lyrics,

            front_cover,
        }
    }
}

/// Lyrics associated with a track.
///
/// `file_name` is the canonical identifier used for persistence. `data` holds
/// lyrics text only transiently — it is populated when lyrics are first
/// extracted from a tag and cleared once written to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lyrics {
    pub file_name: String,
    pub data: String,
}

impl Lyrics {
    /// Flushes buffered lyrics text to the lyrics directory and populates
    /// [`Self::file_name`]. No-op when `data` is already empty.
    pub fn flush(&mut self, file_hash: &str) -> McatResult<()> {
        if !self.data.is_empty() {
            let lrc_name = format!("{file_hash}.lrc");
            let mut lrc_path = config::lrc_dir_path();
            lrc_path.push(&lrc_name);

            fs::write(&lrc_path, &self.data)?;

            self.file_name = lrc_name;
            self.data.clear();
        }

        Ok(())
    }
}

/// Cover image associated with a track.
///
/// `file_name` is the canonical identifier used for persistence. `data` holds
/// raw image bytes only transiently — it is populated when an image is first
/// extracted from a tag and cleared once written to disk.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub mime_type: Option<String>,
    pub description: Option<String>,
    pub file_name: String,
    #[serde(skip)]
    pub data: Vec<u8>,
}

impl Image {
    /// Flushes buffered image bytes to the cover directory and populates
    /// [`Self::file_name`]. No-op when `data` is already empty.
    pub fn flush(&mut self, file_hash: &str) -> McatResult<()> {
        if !self.data.is_empty() {
            let ext = self
                .mime_type
                .as_deref()
                .and_then(|m| m.split('/').next_back())
                .unwrap_or("bin");

            let file_name = format!("{}.{}", file_hash, ext);
            let mut image_path = config::cover_dir_path();
            image_path.push(&file_name);

            std::fs::write(&image_path, &self.data)?;

            self.file_name = file_name;
            self.data.clear();
        }

        Ok(())
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
    pub dates: BTreeSet<NaiveDate>,
    pub track_numbers: BTreeSet<usize>,
    pub disc_numbers: BTreeSet<usize>,
    pub genres: BTreeSet<String>,
    pub composers: BTreeSet<String>,
    pub lyricists: BTreeSet<String>,

    pub hashes: BTreeSet<String>,
}

impl TrackFilter {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        titles: Vec<String>,
        artists: Vec<String>,
        albums: Vec<String>,
        album_artists: Vec<String>,
        dates: Vec<NaiveDate>,
        track_numbers: Vec<usize>,
        disc_numbers: Vec<usize>,
        genres: Vec<String>,
        composers: Vec<String>,
        lyricists: Vec<String>,

        hashes: Vec<String>,
    ) -> Self {
        TrackFilter {
            titles: titles.into_iter().collect(),
            artists: artists.into_iter().collect(),
            albums: albums.into_iter().collect(),
            album_artists: album_artists.into_iter().collect(),
            dates: dates.into_iter().collect(),
            track_numbers: track_numbers.into_iter().collect(),
            disc_numbers: disc_numbers.into_iter().collect(),
            genres: genres.into_iter().collect(),
            composers: composers.into_iter().collect(),
            lyricists: lyricists.into_iter().collect(),

            hashes: hashes.into_iter().collect(),
        }
    }

    /// Applies the filter to the repository, returning hashes of matching
    /// tracks.
    pub fn apply<T: Repo>(self, repo: &T) -> Vec<String> {
        fn matches_opt<T: Ord>(filters: &BTreeSet<T>, value: Option<&T>) -> bool {
            filters.is_empty() || value.is_some_and(|v| filters.contains(v))
        }

        repo.get_track_hashes()
            .into_iter()
            .filter(|hash| matches_opt(&self.hashes, Some(hash)))
            .filter(|hash| {
                let Some(entry) = repo.query_track_by_hash(hash) else {
                    return false;
                };

                matches_opt(&self.titles, entry.tag_attr.title.as_ref())
                    && matches_opt(&self.artists, entry.tag_attr.artist.as_ref())
                    && matches_opt(&self.albums, entry.tag_attr.album.as_ref())
                    && matches_opt(&self.album_artists, entry.tag_attr.album_artist.as_ref())
                    && matches_opt(&self.dates, entry.tag_attr.date.as_ref())
                    && matches_opt(&self.track_numbers, entry.tag_attr.track_number.as_ref())
                    && matches_opt(&self.disc_numbers, entry.tag_attr.disc_number.as_ref())
                    && matches_opt(&self.genres, entry.tag_attr.genre.as_ref())
                    && matches_opt(&self.composers, entry.tag_attr.composer.as_ref())
                    && matches_opt(&self.lyricists, entry.tag_attr.lyricist.as_ref())
            })
            .collect()
    }
}

/// Result of consistency checks between media files and repository records.
#[derive(Serialize, Deserialize)]
pub struct CheckResult {
    /// Hashes found in media files but not tracked in the repository.
    pub not_tracked: BTreeSet<String>,

    /// Hashes tracked in the repository but missing from the media directory.
    pub not_exists: BTreeSet<String>,
}
