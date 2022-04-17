use std::rc::Rc;

use anyhow::Result;
use thiserror::Error;

use twist_common::{
    config::{toml::TomlConfig, ConfigIo},
    files::{git::GitRepository, FileManager, Repository},
    path::Paths,
};

use crate::AddFilesArgs;

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    let paths = Paths::new(args.root_dir);
    let repository: Rc<dyn Repository> = Rc::new(GitRepository::open(
        &paths.root_dir,
        &TomlConfig::file_name(),
    )?);

    let file_manager: FileManager<TomlConfig> = FileManager::new(&paths, &repository);
    file_manager.switch_profile(&args.profile)?;
    file_manager.add_files(args.paths.as_slice())?;
    file_manager.save_config()?;
    file_manager.commit_changes(args.message.unwrap().as_str())?;

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
