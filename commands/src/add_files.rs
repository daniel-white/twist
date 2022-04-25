use std::rc::Rc;

use anyhow::Result;
use thiserror::Error;

use twist_common::{
    config::ConfigManager,
    files::{git::GitRepository, FileManager, Repository},
    path::Paths,
};

use crate::AddFilesArgs;

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    let paths = Rc::new(Paths::new(args.root_dir));
    let config = Rc::new(ConfigManager::open(&paths));

    let repository: Rc<dyn Repository> = Rc::new(GitRepository::open(&config, &paths)?);

    let file_manager: FileManager = FileManager::new(&config, &paths, &repository);
    file_manager.switch_profile(&args.profile)?;
    file_manager.add(&args.paths)?;
    file_manager.save_config()?;
    file_manager.commit_changes(
        args.message
            .unwrap_or_else(|| "Adding new dotfiles".to_string())
            .as_str(),
    )?;

    Ok(())
}
