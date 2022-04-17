use std::rc::Rc;

use anyhow::Result;
use twist_fs::{
    file_manager::FileManager,
    path::Paths,
    repository::{git::GitRepository, Repository},
};

use thiserror::Error;
use twist_shared::{commands::AddFilesArgs, config::toml::TomlConfig};

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    let paths = Paths::new(args.root_dir);
    let repository: Rc<dyn Repository> = Rc::new(GitRepository::open(&paths.root_dir)?);

    let file_manager: FileManager<TomlConfig> = FileManager::new(&paths, &repository);

    file_manager.add_files(args.paths.as_slice())?;
    file_manager.save_config()?;
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
