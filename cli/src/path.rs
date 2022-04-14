use std::fs::metadata;
use std::path::{Path, PathBuf};

use anyhow::Result;
use dirs::home_dir;
use thiserror::Error;
use twist_shared::ROOT_DIR_NAME;

#[derive(Debug, Error)]
enum RootDirError {
    #[error("could not find home directory to use as root directory")]
    HomeDirNotFoundError,
    #[error("the requested root directory is not a directory: {0}")]
    NotADirectoryError(PathBuf),
}

pub fn root_dir(root_dir_override: Option<PathBuf>) -> Result<PathBuf> {
    let root_dir = root_dir_override
        .or_else(|| home_dir().map(|home_dir| Path::new(&home_dir).join(ROOT_DIR_NAME)));

    match root_dir {
        Some(root_dir) => match metadata(&root_dir) {
            Err(_) => Ok(root_dir),
            Ok(metadata) if metadata.is_dir() => Ok(root_dir),
            _ => Err(RootDirError::NotADirectoryError(root_dir).into()),
        },
        None => Err(RootDirError::HomeDirNotFoundError.into()),
    }
}
