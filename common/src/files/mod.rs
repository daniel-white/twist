pub mod git;

use std::fs::copy;

use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;

use dircpy::copy_dir;
use log::debug;

use crate::config::ConfigManager;
use crate::path::{DirPathInfo, FilePathInfo, Paths};

use self::git::GitRepository;

pub struct FileManager {
    paths: Rc<Paths>,
    config: Rc<ConfigManager>,
    repository: Rc<GitRepository>,
}

impl FileManager {
    pub fn new(
        config: &Rc<ConfigManager>,
        paths: &Rc<Paths>,
        repository: &Rc<GitRepository>,
    ) -> Self {
        FileManager {
            config: config.clone(),
            paths: paths.clone(),
            repository: repository.clone(),
        }
    }

    pub fn switch_profile(&self, profile: &str) -> Result<()> {
        self.repository.switch_profile(profile)
    }

    pub fn add(&self, paths: &[PathBuf]) -> Result<()> {
        let (files, dirs) = self.paths.resolve_paths(paths);

        self.add_files(&files)?;
        self.add_dirs(&dirs)?;

        Ok(())
    }

    pub fn add_files(&self, files: &[FilePathInfo]) -> Result<()> {
        for file in files {
            Paths::ensure_parent_dir(&file.full_repo_path)?;

            debug!(
                "copying file {:?} to {:?}",
                file.full_src_path, file.full_repo_path
            );

            copy(&file.full_src_path, &file.full_repo_path)?;
        }

        self.config.add_files(files);

        Ok(())
    }

    fn add_dirs(&self, dirs: &[DirPathInfo]) -> Result<()> {
        for dir in dirs {
            debug!(
                "copying directory {:?} to {:?}",
                dir.full_src_path, dir.repo_path
            );

            Paths::ensure_parent_dir(&dir.full_repo_path)?;
            copy_dir(&dir.full_src_path, &dir.full_repo_path)?;
        }

        self.config.add_dirs(dirs);
        Ok(())
    }

    pub fn save_config(&self) -> Result<()> {
        self.config.save()
    }

    pub fn commit_changes(&self, message: &str) -> Result<()> {
        self.repository.commit(message)
    }
}
