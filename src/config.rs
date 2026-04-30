//! Path-related configuration functions of mcat.

use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;

use crate::errors::McatResult;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug)]
pub struct Config {
    mcat_dir: PathBuf,
    repo_file: PathBuf,
    cover_dir: PathBuf,
    lrc_dir: PathBuf,
    media_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            mcat_dir: PathBuf::from(".mcat/"),
            repo_file: PathBuf::from(".mcat/repo.toml"),
            cover_dir: PathBuf::from(".mcat/images/"),
            lrc_dir: PathBuf::from(".mcat/lyrics/"),
            media_dir: PathBuf::from("media/"),
        }
    }
}

fn get() -> &'static Config {
    CONFIG.get().expect("[FATAL] try to get config before initializing")
}

pub fn init(config: Option<Config>) -> McatResult<()> {
    let config = match config {
        Some(config) => config,
        None => Config::default(),
    };
    fs::remove_dir_all(&config.mcat_dir)?;
    fs::create_dir_all(&config.cover_dir)?;
    fs::create_dir_all(&config.lrc_dir)?;

    CONFIG.set(config).expect("config already initialized");

    Ok(())
}

/// Returns path to repository file.
pub fn repo_file_path() -> PathBuf {
    get().repo_file.clone()
}

/// Returns path to repository backup file.
pub fn repo_backup_file_path() -> PathBuf {
    let mut p = get().repo_file.clone();
    p.set_extension("bak");
    p
}

/// Returns path to cover images directory.
pub fn cover_dir_path() -> PathBuf {
    get().cover_dir.clone()
}

/// Returns path to lyrics directory.
pub fn lrc_dir_path() -> PathBuf {
    get().lrc_dir.clone()
}

/// Returns path to media directory.
pub fn media_dir_path() -> PathBuf {
    get().media_dir.clone()
}
