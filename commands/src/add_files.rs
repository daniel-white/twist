use std::rc::Rc;

use anyhow::Result;
use thiserror::Error;

use twist_common::{
    config::ConfigManager,
    files::{git::GitRepository, FileManager},
    path::Paths,
    DEFAULT_COMMIT_MESSAGE_FOR_ADD,
};

use crate::AddFilesArgs;

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs) -> Result<()> {
    let paths = Rc::new(Paths::new(args.root_dir));
    let config = Rc::new(ConfigManager::open(&paths));

    let repository = GitRepository::open(&paths)?;
    repository.switch_profile(&args.profile)?;

    let file_manager: FileManager = FileManager::new(&config, &paths);
    file_manager.add(&args.paths)?;

    config.save()?;

    repository.commit(
        &args
            .message
            .unwrap_or_else(|| DEFAULT_COMMIT_MESSAGE_FOR_ADD.to_string()),
    )?;

    Ok(())
}
