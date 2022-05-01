use std::fs::{metadata, write};

use anyhow::Result;
use git2::{
    Branch as LibGitBranch, BranchType as LibGitBranchType, Config as LibGitConfig,
    Delta as LibGitDelta, DiffOptions as LibGitDiffOptions, Error as LibGitError,
    Repository as LibGitRepository, RepositoryInitOptions as LibGitRepositoryInitOptions,
    Signature as LibGitSignature, Time as LibGitTime,
};

use log::{debug, info};
use thiserror::Error;
use time::OffsetDateTime;

use crate::{path::Paths, DEFAULT_COMMITTER_EMAIL, DEFAULT_COMMITTER_NAME};

const GIT_CONFIG_USER_NAME: &str = "user.name";
const GIT_CONFIG_USER_EMAIL: &str = "user.email";
const GITIGNORE_FILE_NAME: &str = ".gitignore";
const GITIGNORE_FILE_CONTENT: &str = include_str!("./gitignore.txt");

#[derive(Error, Debug)]
enum RepositoryError {
    #[error("failed to open repository: {0}")]
    InitializeGit(anyhow::Error),
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
    repo: LibGitRepository,
    committer: Committer,
}

impl GitRepository {
    pub fn open(paths: &Paths, profile: &str) -> Result<Self> {
        let root_dir = paths.root_dir.clone();
        debug!("opening repository at {:?}", root_dir);

        let repo = match LibGitRepository::open(&root_dir) {
            Ok(git_repository) => Ok(git_repository),
            Err(_) => {
                let mut opts = LibGitRepositoryInitOptions::new();
                let opts = opts.mkdir(true).initial_head(profile);
                LibGitRepository::init_opts(&root_dir, opts)
            }
        };

        let repo = match repo {
            Ok(repo) => repo,
            Err(err) => return Err(RepositoryError::InitializeGit(err.into()).into()),
        };

        let committer = match repo.config() {
            Ok(mut config) => Committer::new(&mut config),
            Err(err) => {
                debug!("Error reading git config: {:?}", err);
                Committer::default()
            }
        };
        debug!("resolved committer: {:?}", committer);

        debug!("successfully opened repository at {:?}", root_dir);

        let repo = Self { repo, committer };
        repo.switch_profile(profile);

        let gitignore_file_path = root_dir.join(GITIGNORE_FILE_NAME);
        if metadata(&gitignore_file_path).is_err() {
            write(&gitignore_file_path, GITIGNORE_FILE_CONTENT)?
        }

        Ok(repo)
    }

    fn switch_profile(&self, profile: &str) {
        let branch = self.repo.find_branch(profile, LibGitBranchType::Local).ok();

        let result = match branch {
            Some(branch) => self.switch_branch(&branch),
            None => self
                .repo
                .head()
                .and_then(|h| h.peel_to_commit())
                .and_then(|c| self.repo.branch(profile, &c, false))
                .and_then(|b| self.switch_branch(&b)),
        };

        result.unwrap_or_default();
    }

    fn switch_branch(&self, branch: &LibGitBranch) -> Result<(), LibGitError> {
        let name = branch.get().name().unwrap();
        debug!("switching to profile: {}", name);
        self.repo.set_head(name)
    }

    pub fn commit(&self, message: &str) -> Result<()> {
        let now = OffsetDateTime::now_local()?;
        let time = LibGitTime::new(now.unix_timestamp(), now.offset().whole_seconds() / 60);

        let head_commit = match self.repo.head() {
            Ok(head) => head.peel_to_commit().ok(),
            Err(_) => None,
        };

        let head_tree = match head_commit {
            Some(ref commit) => commit.tree().ok(),
            None => None,
        };

        let mut diff_options = LibGitDiffOptions::new();
        diff_options
            .update_index(true)
            .include_untracked(true)
            .recurse_untracked_dirs(true);

        let diff_result = self
            .repo
            .diff_tree_to_workdir(head_tree.as_ref(), Some(&mut diff_options));

        let diff_deltas: Vec<_> = match diff_result {
            Ok(ref diff) => diff.deltas().collect(),
            Err(_) => Vec::new(),
        };

        if diff_deltas.is_empty() {
            info!("no files changed");
            return Ok(());
        }

        let mut index = self.repo.index()?;

        for diff_delta in diff_deltas {
            let delta = diff_delta.status();

            match delta {
                LibGitDelta::Added
                | LibGitDelta::Copied
                | LibGitDelta::Modified
                | LibGitDelta::Renamed
                | LibGitDelta::Untracked
                | LibGitDelta::Unmodified => {
                    let path = diff_delta.new_file().path().unwrap();
                    debug!("Staging {:?} file: {:?}", delta, path);
                    index.add_path(path)?;
                }
                LibGitDelta::Deleted => {
                    let path = diff_delta.old_file().path().unwrap();
                    index.remove_path(path)?;
                }
                _ => debug!("skipping {:?} file", delta),
            }
        }

        let index_oid = index.write_tree()?;
        let index_tree = self.repo.find_tree(index_oid)?;

        let sig = LibGitSignature::new(&self.committer.name, &self.committer.email, &time)?;

        let parents: Vec<_> = [&head_commit].iter().flat_map(|c| c.as_ref()).collect();

        self.repo
            .commit(Some("HEAD"), &sig, &sig, message, &index_tree, &parents)?;

        Ok(())
    }
}
