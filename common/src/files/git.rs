use std::{
    ffi::OsString,
    fs::{metadata, write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use git2::{
    Config as LibGitConfig, Index as LibGitIndex, Repository as LibGitRepository,
    RepositoryInitOptions as LibGitRepositoryInitOptions, Signature as LibGitSignature,
    Status as LibGitStatus, Time as LibGitTime,
};
use glob::glob;
use log::{debug, info};
use thiserror::Error;
use time::OffsetDateTime;

use super::Repository;
use crate::{
    config::ConfigManager,
    path::{DirPathInfo, FilePathInfo, Paths},
    DEFAULT_COMMITTER_EMAIL, DEFAULT_COMMITTER_NAME, DEFAULT_PROFILE,
};

const GIT_CONFIG_USER_NAME: &str = "user.name";
const GIT_CONFIG_USER_EMAIL: &str = "user.email";
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

#[derive(Debug)]
struct Committer {
    name: String,
    email: String,
}

impl Committer {
    fn new(config: &mut LibGitConfig) -> Self {
        let name = config
            .get_string(GIT_CONFIG_USER_NAME)
            .unwrap_or_else(|_| DEFAULT_COMMITTER_NAME.to_string());
        let email = config
            .get_string(GIT_CONFIG_USER_EMAIL)
            .unwrap_or_else(|_| DEFAULT_COMMITTER_EMAIL.to_string());

        Committer { name, email }
    }
}

impl Default for Committer {
    fn default() -> Self {
        Committer {
            name: DEFAULT_COMMITTER_NAME.to_string(),
            email: DEFAULT_COMMITTER_EMAIL.to_string(),
        }
    }
}

pub struct GitRepository {
    config_file_repo_path: PathBuf,
    root_dir: PathBuf,
    repo: LibGitRepository,
    committer: Committer,
}

impl GitRepository {
    pub fn open(config: &ConfigManager, paths: &Paths) -> Result<Self> {
        let root_dir = paths.root_dir.clone();
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

        let committer = match repo.config() {
            Ok(mut config) => Committer::new(&mut config),
            Err(err) => {
                debug!("Error reading git config: {:?}", err);
                Committer::default()
            }
        };

        debug!("resolved committer: {:?}", committer);

        Ok(GitRepository {
            repo,
            root_dir,
            config_file_repo_path: config.config_file_repo_path(),
            committer,
        })
    }

    fn open_index(&self) -> Result<LibGitIndex> {
        debug!("opening index");

        self.repo
            .index()
            .map_err(|err| RepositoryError::OpenIndex(err.into()).into())
    }

    fn add_file_to_index(&self, index: &mut LibGitIndex, p: &Path) -> Result<()> {
        let ignored = self.repo.is_path_ignored(p)?;
        if ignored {
            return Ok(());
        }

        debug!("staging {:?}", self.root_dir.join(p));

        index
            .add_path(p)
            .map_err(|err| RepositoryError::AddFileToIndex(p.to_path_buf(), err.into()).into())
    }
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
            self.add_file_to_index(&mut index, &file.repo_path)?;
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
            self.add_file_to_index(&mut index, file_repo_path)?;
        }

        index
            .write()
            .map_err(|err| RepositoryError::CloseIndex(err.into()).into())
    }

    fn commit(&self, message: &str) -> Result<()> {
        let now = OffsetDateTime::now_local()?;

        let mut index = self.open_index()?;

        debug!("staging config files");
        self.add_file_to_index(&mut index, &self.config_file_repo_path)?;
        self.add_file_to_index(&mut index, Path::new(GITIGNORE_FILE_NAME))?;

        let any_changed_files = index
            .iter()
            .flat_map(|e| String::from_utf8(e.path))
            .map(|e| PathBuf::from(OsString::from(e)))
            .flat_map(|p| self.repo.status_file(&p))
            .any(|s| s != LibGitStatus::CURRENT);

        if !any_changed_files {
            info!("no files changed");
            return Ok(());
        }

        let oid = index.write_tree()?;
        let tree = self.repo.find_tree(oid)?;

        let sig = LibGitSignature::new(
            &self.committer.name,
            &self.committer.email,
            &LibGitTime::new(now.unix_timestamp(), now.offset().whole_seconds() / 60),
        )?;

        let commit = match self.repo.head() {
            Ok(head) => head.peel_to_commit().ok(),
            Err(_) => None,
        };

        let commits: Vec<_> = [&commit].iter().filter_map(|c| c.as_ref()).collect();

        self.repo
            .commit(Some("HEAD"), &sig, &sig, message, &tree, &commits)?;

        Ok(())
    }
}
