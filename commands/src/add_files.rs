use anyhow::Result;
use twist_fs::{file_manager::FileManager, path::Paths, repository::git::GitRepository};

use thiserror::Error;
use twist_shared::commands::AddFilesArgs;

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    let paths = Paths::new(args.root_dir);
    let repository = GitRepository::open(&paths)?;

    let mut file_manager = FileManager::new(&paths, &repository);

    file_manager.add_files(args.paths.as_slice())?;
    repository.commit(args.message.unwrap().as_str())?;
    // let mut file_manager = FileManager::new();
    // let ResolvedPaths {
    //     file_paths,
    //     dir_paths,
    // } = resolve_paths(&args.paths)?;

    // println!("file_paths: {:?}", file_paths);
    // println!("dir_paths: {:?}", dir_paths);

    // file_manager.add_files(&file_paths);
    // file_manager.add_dirs(&dir_paths);

    Ok(())
}
