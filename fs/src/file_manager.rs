use std::cell::RefCell;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::rc::Rc;

use anyhow::Result;

use log::debug;
use twist_shared::config::ConfigIo;

use super::path::Paths;
use super::repository::Repository;

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

    pub fn add_files(&self, paths: &[PathBuf]) -> Result<()> {
        let paths: Vec<_> = paths
            .iter()
            .flat_map(|p| self.paths.resolve_file_paths(p))
            .collect();

        self.repository.add_files(&paths)?;
        self.config.borrow_mut().add_files(&paths);

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
}
