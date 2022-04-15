pub mod git;

use std::{ffi::OsStr, path::Path, rc::Rc};

use anyhow::Result;

use super::path::Paths;

pub trait Repository {
    fn open(paths: &Rc<Paths>) -> Result<Self>
    where
        Self: Sized;

    fn add_files<P: AsRef<Path>>(&self, paths: &[(P, &OsStr)]) -> Result<()>;

    fn add_file<P: AsRef<Path>>(&self, src_path: P, dest_name: &OsStr) -> Result<()> {
        self.add_files(&[(src_path, dest_name)])
    }

    fn commit(&self, message: &str) -> Result<()>;
}
