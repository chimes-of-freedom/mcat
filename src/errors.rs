//! Unified error types and error conversion helpers.

use std::fmt;
use std::io;

// unified error type
#[derive(Debug)]
pub enum McatError {
    FileNotFound,
    TagNotFound,
    TrackNotFound,
    AttrEmpty,
    Io(io::Error),
    Tag(lofty::error::LoftyError),
    TomlDe(toml::de::Error),
    TomlSer(toml::ser::Error),
}

pub type McatResult<T> = Result<T, McatError>;

impl fmt::Display for McatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            McatError::FileNotFound => write!(f, "file not found"),
            McatError::TagNotFound => write!(f, "no tag found in media file"),
            McatError::TrackNotFound => write!(f, "track not found in repo's database"),
            McatError::AttrEmpty => write!(f, "no tag attributes provided"),
            McatError::Io(e) => write!(f, "I/O error: {}", e),
            McatError::Tag(e) => write!(f, "tag operation error: {}", e),
            McatError::TomlDe(e) => write!(f, "failed to parse TOML: {}", e),
            McatError::TomlSer(e) => write!(f, "failed to serialize TOML: {}", e),
        }
    }
}

// TODO: print error chain using `source()`
impl std::error::Error for McatError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            McatError::Io(e) => Some(e),
            McatError::Tag(e) => Some(e),
            McatError::TomlDe(e) => Some(e),
            McatError::TomlSer(e) => Some(e),
            _ => None,
        }
    }
}

impl From<io::Error> for McatError {
    fn from(value: io::Error) -> Self {
        McatError::Io(value)
    }
}

impl From<lofty::error::LoftyError> for McatError {
    fn from(value: lofty::error::LoftyError) -> Self {
        McatError::Tag(value)
    }
}

impl From<toml::de::Error> for McatError {
    fn from(value: toml::de::Error) -> Self {
        McatError::TomlDe(value)
    }
}

impl From<toml::ser::Error> for McatError {
    fn from(value: toml::ser::Error) -> Self {
        McatError::TomlSer(value)
    }
}
