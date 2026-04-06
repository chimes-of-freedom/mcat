//! Path-related configuration functions of mcat.

use std::path::PathBuf;

const REPO_FILE_PATH: &str = ".mcat/repo.toml";
const COVER_DIR_PATH: &str = ".mcat/images/";
const MEDIA_DIR_PATH: &str = "media/";

/// Returns path to repository file.
pub fn repo_file_path() -> PathBuf {
    PathBuf::from(REPO_FILE_PATH)
}

/// Returns path to repository backup file.
pub fn repo_backup_file_path() -> PathBuf {
    let base = PathBuf::from(REPO_FILE_PATH);
    base.with_added_extension("bak")
}

/// Returns path to cover images directory.
pub fn cover_dir_path() -> PathBuf {
    PathBuf::from(COVER_DIR_PATH)
}

/// Returns path to media directory.
pub fn media_dir_path() -> PathBuf {
    PathBuf::from(MEDIA_DIR_PATH)
}
