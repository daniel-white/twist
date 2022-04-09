use std::fs::{canonicalize, metadata};
use std::path::{Path, PathBuf};

use anyhow::Result;
use dirs::home_dir;
use glob::{glob_with, MatchOptions};
use thiserror::Error;

use twist_shared::{DOTFILES_DIR_NAME, ROOT_DIR_NAME};

#[derive(Debug, Error)]
pub enum PathError {
    #[error("could not find home directory")]
    HomeDirNotFoundError,
}

pub fn root_dir() -> Result<PathBuf> {
    match home_dir() {
        Some(home_dir) => Ok(Path::new(&home_dir).join(ROOT_DIR_NAME)),
        None => Err(PathError::HomeDirNotFoundError.into()),
    }
}

pub fn repository_dir<P: AsRef<Path>>(p: P) -> PathBuf {
    p.as_ref().join(DOTFILES_DIR_NAME)
}

#[derive(Debug)]
pub struct ResolvedPaths {
    pub dir_paths: Vec<PathBuf>,
    pub file_paths: Vec<PathBuf>,
}

impl ResolvedPaths {
    fn new() -> Self {
        ResolvedPaths {
            dir_paths: vec![],
            file_paths: vec![],
        }
    }
}

pub fn resolve_paths(paths: &Vec<PathBuf>) -> Result<ResolvedPaths> {
    let root_dir = root_dir()?;
    let root_dir = root_dir.to_str();
    let root_dir = root_dir.ok_or(PathError::HomeDirNotFoundError)?;

    let home_dir: String = match home_dir() {
        Some(home_dir) => home_dir
            .to_str()
            .ok_or(PathError::HomeDirNotFoundError)?
            .to_string(),
        None => return Err(PathError::HomeDirNotFoundError.into()),
    };

    #[cfg(target_os = "macos")]
    let match_options = MatchOptions {
        case_sensitive: false,
        require_literal_leading_dot: true,
        require_literal_separator: false,
    };

    #[cfg(not(target_os = "macos"))]
    let match_options = MatchOptions {
        case_sensitive: true,
        require_literal_leading_dot: true,
        require_literal_separator: false,
    };

    let resolved_paths = paths
        .iter()
        .filter_map(|p| p.to_str())
        .filter(|p| !p.starts_with(&root_dir))
        .filter_map(|p| glob_with(p, match_options).ok())
        .flatten()
        .flatten()
        .filter_map(|p| canonicalize(&p).ok())
        .filter_map(|p| match metadata(&p) {
            Ok(m) => Some((p, m)),
            Err(_) => None,
        })
        .map(|(p, m)| {
            if p.starts_with(&home_dir) {
                (
                    Path::new("~/")
                        .join(p.strip_prefix(&home_dir).unwrap())
                        .to_path_buf(),
                    m,
                )
            } else {
                (p, m)
            }
        })
        .fold(ResolvedPaths::new(), |mut acc, (p, m)| {
            if m.is_dir() {
                acc.dir_paths.push(p);
            } else if m.is_file() {
                acc.file_paths.push(p);
            }
            acc
        });

    Ok(resolved_paths)
}
