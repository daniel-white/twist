use log::debug;

use crate::fs::paths::{hash_path, metadata_file_path};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use toml;

#[derive(Serialize, Deserialize, Debug)]
pub struct Metadata {
    #[serde(default)]
    directories: Directories,
}

#[derive(Serialize, Deserialize, Debug)]
struct Directories {
    #[serde(default)]
    hashes: BTreeMap<String, PathBuf>,
}

impl Metadata {
    pub fn new() -> Self {
        match Metadata::read_from_file() {
            Some(metadata) => metadata,
            None => Metadata::default(),
        }
    }

    fn read_from_file() -> Option<Metadata> {
        let file_path = metadata_file_path();
        debug!("reading metadata from {:?}", file_path);
        match fs::read_to_string(file_path) {
            Ok(contents) => match toml::from_str::<Metadata>(contents.as_str()) {
                Ok(metadata) => Some(metadata),
                Err(err) => {
                    debug!("error parsing metadata file {:?}", err.to_string());
                    None
                }
            },
            Err(err) => {
                debug!("error reading metadata file {:?}", err.to_string());
                None
            }
        }
    }

    pub fn write_to_file(&self) {
        let data = toml::to_string_pretty(self).unwrap();
        fs::write(metadata_file_path(), data).unwrap();
    }

    pub fn add_dir<P: AsRef<Path>>(&mut self, path: P) {
        let hash = hash_path(&path);
        self.directories
            .hashes
            .insert(hash, path.as_ref().to_path_buf());
    }

    pub fn remove_dir<P: AsRef<Path>>(&mut self, path: P) {
        let hash = hash_path(path);
        self.directories.hashes.remove(&hash);
    }
}

impl Default for Metadata {
    fn default() -> Self {
        Metadata {
            directories: Directories::default(),
        }
    }
}

impl Default for Directories {
    fn default() -> Self {
        Directories {
            hashes: BTreeMap::default(),
        }
    }
}
