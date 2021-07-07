use crypto::digest::Digest;
use crypto::sha2::Sha256;
use dirs;
use glob::glob;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_DIR_NAME: &str = ".twist";

pub fn root_dir() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap();
    Path::new(&home_dir).join(ROOT_DIR_NAME)
}

pub fn repo_dir() -> PathBuf {
    root_dir().join("content")
}

pub fn repo_sub_dir_for_dir<P: AsRef<Path>>(p: P) -> PathBuf {
    let hash = hash_path(p);
    repo_dir().join(hash)
}

pub fn metadata_file_path() -> PathBuf {
    repo_dir().join("metadata.toml")
}

pub fn hash_path<P: AsRef<Path>>(p: P) -> String {
    let mut sha256 = Sha256::new();
    sha256.input_str(p.as_ref().to_str().unwrap());
    sha256.result_str()
}

pub fn resolve(paths: &Vec<PathBuf>) -> Vec<(PathBuf, fs::Metadata)> {
    paths
        .iter()
        .map(|p| p.to_str().unwrap())
        .flat_map(|p| glob(p).unwrap())
        .map(|p| p.unwrap())
        .map(|p| fs::canonicalize(p).unwrap())
        .map(|p| (Path::new(&p).to_path_buf(), fs::metadata(p).unwrap()))
        .filter(|(_, m)| m.is_dir() || m.is_file())
        .collect()
}

#[derive(Debug)]
pub struct FileDirInfo {
    storage_dir: PathBuf,
    source_dir: PathBuf,
}

impl FileDirInfo {
    pub fn new(name: &PathBuf) -> Result<Self, Box<dyn Error>> {
        let full_path = name.canonicalize()?;
        let source_dir = full_path.parent().unwrap();

        Ok(FileDirInfo {
            storage_dir: repo_dir().join(hash_path(&source_dir)),
            source_dir: source_dir.to_path_buf(),
        })
    }
}
