use std::path::PathBuf;
use std::rc::Rc;

use anyhow::Result;

use super::path::Paths;
use super::repository::Repository;

pub struct FileManager {
    paths: Rc<Paths>,
    repository: Rc<dyn Repository>,
}

impl FileManager {
    pub fn new(paths: &Rc<Paths>, repository: &Rc<dyn Repository>) -> Self {
        FileManager {
            paths: paths.clone(),
            repository: repository.clone(),
        }
    }

    pub fn add_files(&mut self, paths: &[PathBuf]) -> Result<()> {
        let paths: Vec<_> = paths
            .iter()
            .map(|p| (p.to_owned(), self.paths.file_path(&p)))
            .filter_map(|(src, dest)| dest.map(|dest| (src, dest)))
            .collect();

        self.repository.add_files(paths.as_slice())
    }

    pub fn add_file(&mut self, path: PathBuf) -> Result<()> {
        self.add_files(&[path])
    }
}
