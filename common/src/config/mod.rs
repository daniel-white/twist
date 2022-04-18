pub mod toml;

use std::{
    fmt::Debug,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;

use crate::path::{DirPathInfo, FilePathInfo};

pub trait Config: Sized {
    fn add_file(&mut self, file: &FilePathInfo);

    fn add_files(&mut self, files: &[FilePathInfo]) {
        for file in files {
            self.add_file(file);
        }
    }

    fn remove_file(&mut self, src_path: &Path);

    fn remove_files(&mut self, paths: &[&Path]) {
        for path in paths {
            self.remove_file(path);
        }
    }

    fn add_dir(&mut self, src_path: &DirPathInfo);

    fn add_dirs(&mut self, paths: &[DirPathInfo]) {
        for path in paths {
            self.add_dir(path);
        }
    }

    fn remove_dir(&mut self, src_path: &Path);
}

pub trait ConfigIo: Config + Default + Debug {
    fn file_name() -> PathBuf;

    fn open(reader: &mut dyn Read) -> Result<Self>;

    fn write(&self, writer: &mut impl Write) -> Result<()>;
}
