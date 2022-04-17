use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::prelude::*;
use git2::{
    Index as LibGitIndex, ObjectType as LibGitObjectType, Repository as LibGitRepository,
    RepositoryInitOptions as LibGitRepositoryInitOptions,
};
use log::debug;
use thiserror::Error;

use super::Repository;
use crate::path::FilePathInfo;

#[derive(Error, Debug)]
enum RepositoryError {
    #[error("failed to open repository: {0}")]
    InitializeGit(anyhow::Error),

    #[error("failed to open index: {0}")]
    OpenIndex(anyhow::Error),

    #[error("failed to add file to index: [file={0}, err={1}]")]
    AddFileToIndex(PathBuf, anyhow::Error),

    #[error("failed to close index: {0}")]
    CloseIndex(anyhow::Error),
}

pub struct GitRepository {
    config_file_repo_path: PathBuf,
    git_repository: LibGitRepository,
}

impl GitRepository {
    pub fn open(root_dir: &PathBuf, config_file_repo_path: &Path) -> Result<Self> {
        debug!("opening repository at {:?}", root_dir);

        let git_repository = match LibGitRepository::open(&root_dir) {
            Ok(git_repository) => Ok(git_repository),
            Err(_) => {
                let mut opts = LibGitRepositoryInitOptions::new();
                let opts = opts.mkdir(true);
                LibGitRepository::init_opts(&root_dir, opts)
            }
        };

        let git_repository = match git_repository {
            Ok(git_repository) => git_repository,
            Err(err) => return Err(RepositoryError::InitializeGit(err.into()).into()),
        };

        debug!("successfully opened repository at {:?}", root_dir);

        Ok(GitRepository {
            git_repository,
            config_file_repo_path: config_file_repo_path.to_path_buf(),
        })
    }

    fn open_index(&self) -> Result<LibGitIndex> {
        debug!("opening index");

        self.git_repository
            .index()
            .map_err(|err| RepositoryError::OpenIndex(err.into()).into())
    }
}

fn add_file_to_index(index: &mut LibGitIndex, file: &Path) -> Result<()> {
    index
        .add_path(file)
        .map_err(|err| RepositoryError::AddFileToIndex(file.to_path_buf(), err.into()).into())
}

impl Repository for GitRepository {
    fn add_files(&self, files: &[FilePathInfo]) -> Result<()> {
        let mut index = self.open_index()?;

        for file in files {
            debug!("adding {:?} as {:?}", file.full_src_path, file.repo_path);
            add_file_to_index(&mut index, &file.repo_path)?;
        }

        index
            .write()
            .map_err(|err| RepositoryError::CloseIndex(err.into()))?;

        Ok(())
    }

    fn commit(&self, message: &str) -> Result<()> {
        let now: DateTime<Local> = Local::now();
        let mut index = self.open_index()?;
        add_file_to_index(&mut index, &self.config_file_repo_path)?;
        index
            .write()
            .map_err(|err| RepositoryError::CloseIndex(err.into()))?;

        let tree = index.write_tree()?;
        let tree = self.git_repository.find_tree(tree)?;
        let sig = git2::Signature::new(
            "Example",
            "Example",
            &git2::Time::new(now.timestamp(), now.offset().local_minus_utc()),
        )?;
        let commit = self
            .git_repository
            .head()?
            .resolve()?
            .peel(LibGitObjectType::Commit)
            .and_then(|obj| {
                obj.into_commit()
                    .map_err(|_| git2::Error::from_str("Couldn't find commit"))
            })?;
        self.git_repository
            .commit(Some("HEAD"), &sig, &sig, message, &tree, &[&commit])?;

        Ok(())
    }

    // fn switch_profile(&self, profile_name: &str) -> Result<()> {
    //     // let branch = self
    //     //     .git_repository
    //     //     .find_branch(profile_name, GitBranchType::Local)
    //     //     .or_else(||
    //     //          self.git_repository.branch(profile_name, None))?;
    // }
}
