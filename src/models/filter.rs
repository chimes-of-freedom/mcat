//! Track filter.

use std::collections::HashSet;

pub struct TrackFilter {
    pub ids: HashSet<i64>,
    pub titles: HashSet<String>,
    pub artists: HashSet<String>,
    pub albums: HashSet<String>,
    pub album_artists: HashSet<String>,
    pub recording_dates: HashSet<String>,
    pub release_dates: HashSet<String>,
    pub track_numbers: HashSet<i64>,
    pub disc_numbers: HashSet<i64>,
    pub genres: HashSet<String>,
    pub composers: HashSet<String>,
    pub lyricists: HashSet<String>,
}

impl TrackFilter {
    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
            && self.titles.is_empty()
            && self.artists.is_empty()
            && self.albums.is_empty()
            && self.album_artists.is_empty()
            && self.recording_dates.is_empty()
            && self.release_dates.is_empty()
            && self.track_numbers.is_empty()
            && self.disc_numbers.is_empty()
            && self.genres.is_empty()
            && self.composers.is_empty()
            && self.lyricists.is_empty()
    }
}
