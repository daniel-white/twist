pub mod git;

use std::fs::copy;

use std::path::PathBuf;
use std::rc::Rc;

use anyhow::{Ok, Result};

use dircpy::copy_dir;
use log::debug;

use crate::config::ConfigManager;
use crate::path::{DirPathInfo, FilePathInfo, Paths};

pub struct FileManager {
    paths: Rc<Paths>,
    config: Rc<ConfigManager>,
}

impl FileManager {
    pub fn new(config: &Rc<ConfigManager>, paths: &Rc<Paths>) -> Self {
        FileManager {
            config: config.clone(),
            paths: paths.clone(),
        }
    }

    pub fn add(&self, paths: &[PathBuf]) -> Result<()> {
        let (files, dirs) = self.paths.resolve_paths(paths);

        self.add_files(&files)?;
        self.add_dirs(&dirs)?;

        Ok(())
    }

    pub fn update(&self) -> Result<()> {
        let files = self.config.files();
        self.copy_files_to_repo(&files)?;

        let dirs = self.config.dirs();
        self.copy_dirs_to_repo(&dirs)?;
        Ok(())
    }

    fn add_files(&self, files: &[FilePathInfo]) -> Result<()> {
        if !files.is_empty() {
            self.copy_files_to_repo(files)?;
            self.config.add_files(files);
        }

        Ok(())
    }

    fn copy_files_to_repo(&self, files: &[FilePathInfo]) -> Result<()> {
        for file in files {
            Paths::ensure_parent_dir(&file.full_repo_path)?;

            debug!(
                "copying file {:?} to {:?}",
                file.full_src_path, file.full_repo_path
            );

            copy(&file.full_src_path, &file.full_repo_path)?;
        }

        Ok(())
    }

    fn add_dirs(&self, dirs: &[DirPathInfo]) -> Result<()> {
        if !dirs.is_empty() {
            self.copy_dirs_to_repo(dirs)?;
            self.config.add_dirs(dirs);
        }
        Ok(())
    }

    fn copy_dirs_to_repo(&self, dirs: &[DirPathInfo]) -> Result<()> {
        for dir in dirs {
            debug!(
                "copying directory {:?} to {:?}",
                dir.full_src_path, dir.repo_path
            );

            Paths::ensure_parent_dir(&dir.full_repo_path)?;
            copy_dir(&dir.full_src_path, &dir.full_repo_path)?;
        }

        Ok(())
    }
}
