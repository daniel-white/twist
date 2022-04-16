pub mod git;

use std::path::PathBuf;

use anyhow::Result;

pub trait Repository {
    fn add_files(&self, paths: &[(PathBuf, PathBuf)]) -> Result<()>;

    fn add_file(&self, src_path: PathBuf, dest_name: PathBuf) -> Result<()> {
        self.add_files(&[(src_path, dest_name)])
    }

    fn commit(&self, message: &str) -> Result<()>;
}
