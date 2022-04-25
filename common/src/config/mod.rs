mod toml;

use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use log::debug;
use serde::{Deserialize, Serialize};

use self::toml::TomlConfigFilePersistence;
use crate::path::{DirPathInfo, FilePathInfo, Paths};

#[derive(Serialize, Deserialize, Debug, Default)]
pub(self) struct ConfigDirsData(BTreeMap<PathBuf, PathBuf>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub(self) struct ConfigFilesData(BTreeMap<PathBuf, PathBuf>);

#[derive(Serialize, Deserialize, Debug, Default)]
pub(self) struct ConfigData {
    #[serde(default)]
    dirs: ConfigDirsData,
    #[serde(default)]
    files: ConfigFilesData,
}

impl ConfigData {
    pub fn add_dir(&mut self, dir: &DirPathInfo) {
        self.dirs
            .0
            .insert(dir.src_path.clone(), dir.config_repo_path.clone());
    }

    pub fn remove_dir(&mut self, path: &Path) {
        self.dirs.0.remove(path);
    }

    pub fn add_file(&mut self, file: &FilePathInfo) {
        self.files
            .0
            .insert(file.src_path.clone(), file.config_repo_path.clone());
    }

    pub fn remove_file(&mut self, path: &Path) {
        self.files.0.remove(path);
    }
}

pub(self) trait ConfigFilePersistence: Sized {
    fn file_name(&self) -> PathBuf
    where
        Self: Sized;

    fn read(&self, reader: &mut impl Read) -> Result<ConfigData>
    where
        Self: Sized;

    fn write(&self, config: &ConfigData, writer: &mut impl Write) -> Result<()>
    where
        Self: Sized;
}

pub struct ConfigManager {
    config_data: RefCell<ConfigData>,
    persistence: TomlConfigFilePersistence,
    config_file_path: PathBuf,
}

impl ConfigManager {
    pub fn open(paths: &Paths) -> Self {
        let persistence = TomlConfigFilePersistence::default();
        let config_file_path = paths.root_dir.join(persistence.file_name());
        debug!("reading configuration from {:?}", config_file_path);

        let config_data = match File::open(&config_file_path) {
            Ok(file) => {
                let mut reader = BufReader::new(file);
                persistence.read(&mut reader).unwrap_or_default()
            }
            _ => ConfigData::default(),
        };

        Self {
            config_data: RefCell::new(config_data),
            persistence,
            config_file_path,
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.config_file_path)?;

        let mut writer = BufWriter::new(config_file);

        self.persistence
            .write(&self.config_data.borrow(), &mut writer)
    }

    pub fn config_file_repo_path(&self) -> PathBuf {
        self.persistence.file_name()
    }

    pub fn add_files(&self, files: &[FilePathInfo]) {
        for file in files {
            self.config_data.borrow_mut().add_file(file);
        }
    }

    pub fn remove_files(&self, paths: &[&Path]) {
        for path in paths {
            self.config_data.borrow_mut().remove_file(path);
        }
    }

    pub fn add_dirs(&self, paths: &[DirPathInfo]) {
        for path in paths {
            self.config_data.borrow_mut().add_dir(path);
        }
    }

    pub fn remove_dirs(self, paths: &[&Path]) {
        for path in paths {
            self.config_data.borrow_mut().remove_dir(path);
        }
    }
}
