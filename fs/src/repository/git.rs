use std::fs::copy;
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;
use chrono::prelude::*;
use git2::{
    ObjectType as LibGitObjectType, Repository as LibGitRepository,
    RepositoryInitOptions as LibGitRepositoryInitOptions,
};
use log::debug;
use thiserror::Error;

use crate::path::Paths;

use super::Repository;

#[derive(Error, Debug)]
enum RepositoryError {
    #[error("failed to open repository: {0}")]
    InitializeGit(anyhow::Error),

    #[error("failed to create index: {0}")]
    CreateIndex(anyhow::Error),

    // #[error("failed switch profile: {0}")]
    // SwitchProfile(anyhow::Error),

    // #[error("repository path ({0}) is not a directory")]
    // NotADirectory(PathBuf),
    #[error("unable to add file from {0}: {1}")]
    AddFile(PathBuf, anyhow::Error),
}

pub struct GitRepository {
    paths: Rc<Paths>,
    git_repository: LibGitRepository,
}

impl GitRepository {
    pub fn open(paths: &Rc<Paths>) -> Result<Rc<dyn Repository>> {
        debug!("opening repository at {:?}", paths.root_dir);

        let git_repository = match LibGitRepository::open(&paths.root_dir) {
            Ok(git_repository) => Ok(git_repository),
            Err(_) => {
                let mut opts = LibGitRepositoryInitOptions::new();
                let opts = opts.mkdir(true);
                LibGitRepository::init_opts(&paths.root_dir, opts)
            }
        };

        let git_repository = match git_repository {
            Ok(git_repository) => git_repository,
            Err(err) => return Err(RepositoryError::InitializeGit(err.into()).into()),
        };

        debug!("successfully opened repository at {:?}", paths.root_dir);

        Ok(Rc::new(GitRepository {
            paths: paths.clone(),
            git_repository,
        }))
    }
}

impl Repository for GitRepository {
    fn add_files(&self, paths: &[(PathBuf, PathBuf)]) -> Result<()> {
        let mut git_index = self
            .git_repository
            .index()
            .map_err(|err| RepositoryError::CreateIndex(err.into()))?;

        for (src_path, dest_path) in paths {
            debug!("adding {:?} as {:?}", src_path, dest_path);

            Paths::ensure_parent_dir(&dest_path);

            copy(&src_path, &dest_path)
                .map_err(|err| RepositoryError::AddFile(src_path.to_path_buf(), err.into()))?;

            let index_path = dest_path.strip_prefix(&self.paths.root_dir)?;

            git_index.add_path(index_path)?;
        }

        git_index.write()?;
        //  {
        //     return Err(anyhow::Error::new("git fail".into()));
        // }
        Ok(())
    }

    fn commit(&self, message: &str) -> Result<()> {
        let now: DateTime<Local> = Local::now();
        let mut git_index = self.git_repository.index()?;
        let tree = git_index.write_tree()?;
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
