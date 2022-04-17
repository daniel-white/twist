use anyhow::Result;
//use twist_fs::path::{resolve_paths, ResolvedPaths};

use thiserror::Error;
use twist_common::commands::RemoveFilesArgs;

#[derive(Error, Debug)]
enum RemoveFilesError {}

pub fn remove_files(_args: RemoveFilesArgs) -> Result<()> {
    // // let _repository = Repository::open(repo_dir())?;
    // // let mut file_manager = FileManager::new();
    // let ResolvedPaths {
    //     file_paths,
    //     dir_paths,
    // } = resolve_paths(&args.paths)?;

    // println!("file_paths: {:?}", file_paths);
    // println!("dir_paths: {:?}", dir_paths);

    // // file_manager.remove_files(&file_paths);
    // // file_manager.remove_dirs(&dir_paths);

    Ok(())
}
