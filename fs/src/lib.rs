pub mod file_manager;
pub mod path;
pub mod repository;

// use std::{path::PathBuf, rc::Rc};

// use path::Paths;

// pub struct FileRef {
//     pub src_full_path: PathBuf,
//     pub src_friendly_path: PathBuf,
//     pub dest_path: PathBuf,
//     pub dest_repo_path: PathBuf,
// }

// pub struct FileRefFactory {
//     paths: Rc<Paths>,
// }

// impl FileRefFactory {
//     pub fn new(paths: &Rc<Paths>) -> Self {
//         FileRefFactory {
//             paths: paths.clone(),
//         }
//     }

//     pub fn create(&self, src_path: &Path) -> FileRef {
//         FileRef {
//             src_path: self.paths.file_repo_path(src_path).unwrap(),
//             dest_path: self.paths.file_repo_path(dest_name).unwrap(),
//         }
//     }
// }
