pub mod common;
pub mod display;
pub mod edit;

#[derive(Debug)]
pub enum McatError {
    FileNotFound,
    OpenFailed,
    ReadFailed,
    TagNotFound,
    WriteFailed,

    AttrEmpty,
}

// should sync with members in `Edit`
#[derive(Debug)]
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
}
