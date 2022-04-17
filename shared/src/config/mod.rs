pub mod toml;

use std::{
    ffi::OsString,
    fmt::Debug,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::Result;

pub trait Config: Sized {
    fn add_file(&mut self, src_path: &Path, dest_name: &Path);

    fn add_files(&mut self, paths: &[(PathBuf, PathBuf)]) {
        for (src_path, dest_name) in paths {
            self.add_file(src_path, dest_name);
        }
    }

    fn remove_file(&mut self, src_path: &Path);

    fn remove_files(&mut self, paths: &[&Path]) {
        for path in paths {
            self.remove_file(path);
        }
    }

    fn add_dir(&mut self, src_path: &Path);

    fn remove_dir(&mut self, src_path: &Path);
}

pub trait ConfigIo: Config + Default + Debug {
    fn file_name() -> OsString;

    fn open(reader: &mut dyn Read) -> Result<Self>;

    fn write(&self, writer: &mut impl Write) -> Result<()>;
}
