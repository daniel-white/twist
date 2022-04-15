use std::path::Path;

use anyhow::Result;
use twist_fs::{
    path::Paths,
    repository::{git::GitRepository, Repository},
};

use thiserror::Error;
use twist_shared::commands::AddFilesArgs;

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    let paths = Paths::new(args.root_dir);
    let _repository = GitRepository::open(&paths)?;

    _repository.add_file(
        args.paths.first().unwrap(),
        Path::new(args.paths.first().unwrap()).file_name().unwrap(),
    )?;
    _repository.commit(args.message.unwrap().as_str())?;
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
