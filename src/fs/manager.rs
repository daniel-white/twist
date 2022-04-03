use crate::data::metadata::Metadata;
use crate::fs::ensure_dir;
use crate::fs::paths::repo_sub_dir_for_dir;
use std::fs;
use std::path::Path;

pub struct FileManager {
    metadata: Metadata,
}

impl FileManager {
    pub fn new() -> Self {
        FileManager {
            metadata: Metadata::new(),
        }
    }

    pub fn add_files<P: AsRef<Path>>(&mut self, paths: &[P]) {
        for p in paths {
            self.add_file(p);
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P) {
        let parent_dir = path.as_ref().parent().unwrap();
        self.metadata.add_dir(parent_dir);
        let dest_dir = repo_sub_dir_for_dir(&parent_dir);
        ensure_dir(&dest_dir);
        let file_name = path.as_ref().file_name().unwrap();
        let dest_path = dest_dir.join(file_name);
        fs::copy(path, dest_path).unwrap();
    }

    pub fn remove_files<P: AsRef<Path>>(&mut self, paths: &[P]) {
        for p in paths {
            self.remove_file(p);
        }
    }

    pub fn remove_file<P: AsRef<Path>>(&mut self, _path: P) {}

    pub fn add_dirs<P: AsRef<Path>>(&mut self, paths: &[P]) {
        for p in paths {
            self.add_dir(p);
        }
    }

    pub fn add_dir<P: AsRef<Path>>(&mut self, _path: P) {}

    pub fn remove_dirs<P: AsRef<Path>>(&mut self, paths: &[P]) {
        for p in paths {
            self.remove_dir(p);
        }
    }

    pub fn remove_dir<P: AsRef<Path>>(&mut self, _path: P) {}
}

impl Drop for FileManager {
    fn drop(&mut self) {
        self.metadata.write_to_file();
    }
}
