use std::{
    fs::{metadata, write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use git2::{
    Index as LibGitIndex, Repository as LibGitRepository,
    RepositoryInitOptions as LibGitRepositoryInitOptions,
};
use glob::glob;
use log::debug;
use thiserror::Error;
use time::OffsetDateTime;

use super::Repository;
use crate::{
    path::{DirPathInfo, FilePathInfo},
    DEFAULT_PROFILE,
};

const GITIGNORE_FILE_NAME: &str = ".gitignore";
const GITIGNORE_FILE_CONTENT: &str = include_str!("./gitignore.txt");

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
    root_dir: PathBuf,
    repo: LibGitRepository,
}

impl GitRepository {
    pub fn open(root_dir: &PathBuf, config_file_repo_path: &Path) -> Result<Self> {
        debug!("opening repository at {:?}", root_dir);

        let repo = match LibGitRepository::open(&root_dir) {
            Ok(git_repository) => Ok(git_repository),
            Err(_) => {
                let mut opts = LibGitRepositoryInitOptions::new();
                let opts = opts.mkdir(true).initial_head(DEFAULT_PROFILE);
                LibGitRepository::init_opts(&root_dir, opts)
            }
        };

        let repo = match repo {
            Ok(repo) => repo,
            Err(err) => return Err(RepositoryError::InitializeGit(err.into()).into()),
        };

        debug!("successfully opened repository at {:?}", root_dir);

        let gitignore_file_path = root_dir.join(GITIGNORE_FILE_NAME);
        if metadata(&gitignore_file_path).is_err() {
            write(&gitignore_file_path, GITIGNORE_FILE_CONTENT)?
        }

        Ok(GitRepository {
            repo,
            root_dir: root_dir.to_owned(),
            config_file_repo_path: config_file_repo_path.to_path_buf(),
        })
    }

    fn open_index(&self) -> Result<LibGitIndex> {
        debug!("opening index");

        self.repo
            .index()
            .map_err(|err| RepositoryError::OpenIndex(err.into()).into())
    }
}

fn add_file_to_index(repo: &LibGitRepository, index: &mut LibGitIndex, file: &Path) -> Result<()> {
    let ignored = repo.is_path_ignored(file)?;
    if ignored {
        return Ok(());
    }

    index
        .add_path(file)
        .map_err(|err| RepositoryError::AddFileToIndex(file.to_path_buf(), err.into()).into())
}

impl Repository for GitRepository {
    fn switch_profile(&self, profile: &str) -> Result<()> {
        debug!("switching to profile: {}", profile);

        // debug!("resolving HEAD commit");
        // let head = match self.repo.head() {
        //     Ok(head) => head.resolve().ok(),
        //     Err(_) => None,
        // };

        // let commit = match head {
        //     Some(head) => head
        //         .peel(LibGitObjectType::Commit)
        //         .map(|obj| obj.clone().as_commit().unwrap())
        //         .ok(),
        //     None => None,
        // };

        // debug!("switching branch");

        // self.repo
        //     .find_branch(profile, LibGitBranchType::Local)
        //     .or_else(|_| self.repo.branch(profile, commit.unwrap(), false))
        //     .unwrap();

        Ok(())
    }

    fn add_files(&self, files: &[FilePathInfo]) -> Result<()> {
        let mut index = self.open_index()?;

        for file in files {
            debug!("adding {:?} as {:?}", file.full_src_path, file.repo_path);
            add_file_to_index(&self.repo, &mut index, &file.repo_path)?;
        }

        index
            .write()
            .map_err(|err| RepositoryError::CloseIndex(err.into()).into())
    }

    fn add_dirs(&self, dirs: &[DirPathInfo]) -> Result<()> {
        let mut index = self.open_index()?;

        for file_path in dirs
            .iter()
            .flat_map(|dir| glob(dir.full_repo_path.join("**/*").to_str().unwrap()))
            .flatten()
            .flatten()
            .flat_map(|p| match metadata(&p) {
                Ok(m) if m.is_file() => Some(p),
                _ => None,
            })
        {
            let file_repo_path = file_path.strip_prefix(&self.root_dir).unwrap();
            debug!("adding {:?} as {:?}", file_path, file_repo_path);
            add_file_to_index(&self.repo, &mut index, file_repo_path)?;
        }

        index
            .write()
            .map_err(|err| RepositoryError::CloseIndex(err.into()).into())
    }

    fn commit(&self, message: &str) -> Result<()> {
        let now = OffsetDateTime::now_local()?;
        let mut index = self.open_index()?;
        add_file_to_index(&self.repo, &mut index, &self.config_file_repo_path)?;
        add_file_to_index(&self.repo, &mut index, Path::new(GITIGNORE_FILE_NAME))?;
        index
            .write()
            .map_err(|err| RepositoryError::CloseIndex(err.into()))?;

        let oid = index.write_tree()?;
        //let commit = self.repo.head()?.peel_to_commit()?;
        let tree = self.repo.find_tree(oid)?;

        let sig = git2::Signature::new(
            "Example",
            "Example",
            &git2::Time::new(now.unix_timestamp(), now.offset().whole_seconds()),
        )?;

        if let Ok(commit) = self.repo.head()?.peel_to_commit() {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[&commit])?;
        } else {
            self.repo
                .commit(Some("HEAD"), &sig, &sig, message, &tree, &[])?;
        }

        Ok(())
    }
}
