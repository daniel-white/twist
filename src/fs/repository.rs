use std::fs::{create_dir_all, metadata};
use std::path::{Path, PathBuf};

use anyhow::Result;
use git2::Repository as GitRepository;
use log::debug;
use thiserror::Error;

#[derive(Error, Debug)]
enum RepositoryError {
    #[error("failed to initialize the repository: {0}")]
    InitializationError(anyhow::Error),

    #[error("repository path ({0}) is not a directory")]
    NotADirectoryError(PathBuf),
}

fn ensure_dir<P: AsRef<Path>>(p: P) -> Result<()> {
    match metadata(&p) {
        Ok(metadata) if (metadata.is_dir()) => {
            debug!("the {} directory exists", p.as_ref().display());
            Ok(())
        }
        Ok(_) => Err(RepositoryError::NotADirectoryError(
            p.as_ref().clone().to_path_buf(),
        ))?,
        _ => {
            debug!("creating {} directory", p.as_ref().display());
            create_dir_all(&p)
                .map_err(|err| RepositoryError::InitializationError(err.into()).into())
        }
    }
}

pub struct Repository {
    git_repository: GitRepository,
}

impl Repository {
    pub fn open<P: AsRef<Path>>(p: P) -> Result<Repository> {
        ensure_dir(p)
            .and_then(|_| GitRepository::open(p.as_ref()))
            .map_err(|err| RepositoryError::InitializationError(err.into()).into())
            .or_else(|_| GitRepository::init(p.as_ref()))
            .map_err(|err| RepositoryError::InitializationError(err.into()).into())
            .map(|git_repository| Repository { git_repository })
    }
}
