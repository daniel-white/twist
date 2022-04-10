use anyhow::Result;
use twist_fs::path::{resolve_paths, ResolvedPaths};

use thiserror::Error;
use twist_shared::commands::AddFilesArgs;

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    // let _repository = Repository::open(repo_dir())?;
    // let mut file_manager = FileManager::new();
    let ResolvedPaths {
        file_paths,
        dir_paths,
    } = resolve_paths(&args.paths)?;

    println!("file_paths: {:?}", file_paths);
    println!("dir_paths: {:?}", dir_paths);

    // file_manager.add_files(&file_paths);
    // file_manager.add_dirs(&dir_paths);

    Ok(())
}
