pub mod manager;
pub mod paths;
pub mod repository;

use std::fs::{create_dir_all, metadata};
use std::path::Path;

pub fn ensure_dir<P: AsRef<Path>>(p: P) {
    match metadata(&p) {
        Ok(metadata) if (metadata.is_file()) => panic!("file exists"),
        _ => create_dir_all(&p).unwrap(),
    }
}
