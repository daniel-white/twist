// use crypto::digest::Digest;
// use crypto::sha2::Sha256;
// use dirs;
// use glob::glob;
// use std::error::Error;
// use std::fs;
// use std::path::{Path, PathBuf};

// const ROOT_DIR_NAME: &str = ".twist";

// pub fn root_dir() -> PathBuf {
//     let home_dir = dirs::home_dir().unwrap();
//     Path::new(&home_dir).join(ROOT_DIR_NAME)
// }

// pub fn repo_dir() -> PathBuf {
//     root_dir().join("content")
// }

// pub fn repo_sub_dir_for_dir<P: AsRef<Path>>(p: P) -> PathBuf {
//     let hash = hash_path(p);
//     repo_dir().join(hash)
// }

// pub fn metadata_file_path() -> PathBuf {
//     repo_dir().join("metadata.toml")
// }

// pub fn hash_path<P: AsRef<Path>>(p: P) -> String {
//     let mut sha256 = Sha256::new();
//     sha256.input_str(p.as_ref().to_str().unwrap());
//     sha256.result_str()
// }

// #[derive(Debug)]
// pub struct ResolvedPaths {
//     pub dir_paths: Vec<PathBuf>,
//     pub file_paths: Vec<PathBuf>,
// }

// impl ResolvedPaths {
//     fn new() -> Self {
//         ResolvedPaths {
//             dir_paths: vec![],
//             file_paths: vec![],
//         }
//     }
// }

// pub fn resolve_paths(paths: &Vec<PathBuf>) -> ResolvedPaths {
//     paths
//         .iter()
//         .filter_map(|p| p.to_str())
//         .filter_map(|p| glob(p).ok())
//         .flatten()
//         .flatten()
//         .filter_map(|p| fs::canonicalize(&p).ok())
//         .filter_map(|p| fs::metadata(&p).ok().and_then(|m| Some((p, m))))
//         .fold(ResolvedPaths::new(), |mut acc, (p, m)| {
//             if m.is_dir() {
//                 acc.dir_paths.push(p);
//             } else if m.is_file() {
//                 acc.file_paths.push(p);
//             }
//             acc
//         })
// }

// #[derive(Debug)]
// pub struct FileDirInfo {
//     storage_dir: PathBuf,
//     source_dir: PathBuf,
// }

// impl FileDirInfo {
//     pub fn new(name: &PathBuf) -> Result<Self, Box<dyn Error>> {
//         let full_path = name.canonicalize()?;
//         let source_dir = full_path.parent().unwrap();

//         Ok(FileDirInfo {
//             storage_dir: repo_dir().join(hash_path(&source_dir)),
//             source_dir: source_dir.to_path_buf(),
//         })
//     }
// }
