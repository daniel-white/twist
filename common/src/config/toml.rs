use std::{
    collections::{BTreeMap, BTreeSet},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;
use serde::{Deserialize, Serialize};
use toml::{from_str as from_toml_str, to_string_pretty as to_toml_string};

use crate::path::FilePathInfo;

use super::{Config, ConfigIo};

pub const CONFIG_FILE_NAME: &str = "config.toml";

#[derive(Serialize, Deserialize, Debug)]
pub struct TomlConfig {
    #[serde(default)]
    dirs: TomlConfigDirs,
    #[serde(default)]
    files: TomlConfigFiles,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct TomlConfigDirs(BTreeSet<PathBuf>);

#[derive(Serialize, Deserialize, Debug, Default)]
struct TomlConfigFiles(BTreeMap<PathBuf, PathBuf>);

impl Config for TomlConfig {
    fn add_dir(&mut self, path: &Path) {
        self.dirs.0.insert(path.to_path_buf());
    }

    fn remove_dir(&mut self, path: &Path) {
        self.dirs.0.remove(path);
    }

    fn add_file(&mut self, file: &FilePathInfo) {
        self.files
            .0
            .insert(file.src_path.clone(), file.config_repo_path.clone());
    }

    fn remove_file(&mut self, path: &Path) {
        self.files.0.remove(path);
    }
}

impl ConfigIo for TomlConfig {
    fn file_name() -> PathBuf {
        PathBuf::from(CONFIG_FILE_NAME)
    }

    fn open(reader: &mut dyn Read) -> Result<Self> {
        let mut toml = String::new();
        reader.read_to_string(&mut toml)?;
        Ok(from_toml_str(&toml)?)
    }

    fn write(&self, writer: &mut impl Write) -> Result<()> {
        let toml = to_toml_string(self)?;
        write!(writer, "{}", toml)?;
        Ok(())
    }
}

impl Default for TomlConfig {
    fn default() -> Self {
        TomlConfig {
            dirs: TomlConfigDirs(BTreeSet::new()),
            files: TomlConfigFiles(BTreeMap::new()),
        }
    }
}
