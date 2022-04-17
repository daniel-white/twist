pub mod git;

use std::cell::RefCell;
use std::fs::{copy, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Result;

use log::debug;

use crate::config::ConfigIo;
use crate::path::{FilePathInfo, Paths};

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

    pub fn add_files(&self, paths: &[PathBuf]) -> Result<()> {
        let files: Vec<_> = paths
            .iter()
            .flat_map(|p| self.paths.resolve_file_paths(p))
            .collect();

        for file in &files {
            debug!(
                "copying file {:?} to {:?}",
                file.full_src_path, file.full_repo_path
            );

            Paths::ensure_parent_dir(&file.full_repo_path);
            copy(&file.full_src_path, &file.full_repo_path)?;
        }

        self.repository.add_files(&files)?;
        self.config.borrow_mut().add_files(&files);

        Ok(())
    }

    pub fn add_file(&self, path: &Path) -> Result<()> {
        self.add_files(&[PathBuf::from(path)])
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
