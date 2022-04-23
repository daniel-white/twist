pub mod git;

use std::cell::RefCell;
use std::fs::{copy, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;

use dircpy::copy_dir;
use log::debug;

use crate::config::ConfigIo;
use crate::path::{DirPathInfo, FilePathInfo, Paths};

pub trait Repository {
    fn switch_profile(&self, profile: &str) -> Result<()>;

    fn add_files(&self, files: &[FilePathInfo]) -> Result<()>;

    fn commit(&self, message: &str) -> Result<()>;
}

pub struct FileManager<C>
where
    C: ConfigIo,
{
    paths: Rc<Paths>,
    config_file_path: PathBuf,
    config: RefCell<C>,
    repository: Rc<dyn Repository>,
}

impl<C> FileManager<C>
where
    C: ConfigIo,
{
    pub fn new(paths: &Rc<Paths>, repository: &Rc<dyn Repository>) -> Self {
        let config_file_path = paths.root_dir.join(C::file_name());

        debug!("reading configuration from {:?}", config_file_path);
        let config = match File::open(&config_file_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                C::open(&mut reader).ok().or_else(|| Some(C::default()))
            }
            Err(_) => Some(C::default()),
        }
        .unwrap();

        FileManager {
            paths: paths.clone(),
            config_file_path,
            config: RefCell::new(config),
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

    fn add_files(&self, files: &[FilePathInfo]) -> Result<()> {
        for file in files {
            Paths::ensure_parent_dir(&file.full_repo_path)?;

            debug!(
                "copying file {:?} to {:?}",
                file.full_src_path, file.full_repo_path
            );

            copy(&file.full_src_path, &file.full_repo_path)?;
        }

        self.repository.add_files(files)?;
        self.config.borrow_mut().add_files(files);

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

        self.config.borrow_mut().add_dirs(dirs);
        Ok(())
    }

    pub fn save_config(&self) -> Result<()> {
        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.config_file_path)?;

        self.config
            .borrow_mut()
            .write(&mut BufWriter::new(config_file))
    }

    pub fn commit_changes(&self, message: &str) -> Result<()> {
        self.repository.commit(message)
    }
}
