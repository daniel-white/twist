use std::rc::Rc;

use anyhow::Result;
use thiserror::Error;

use twist_common::{
    config::ConfigManager,
    files::{git::GitRepository, FileManager},
    path::Paths,
};

use crate::UpdateRepositoryArgs;

#[derive(Error, Debug)]
enum UpdateRepositoryError {}

pub fn update_repository(args: UpdateRepositoryArgs) -> Result<()> {
    let paths = Rc::new(Paths::new(args.root_dir));
    let config = Rc::new(ConfigManager::open(&paths));

    let repository = GitRepository::open(&paths)?;
    repository.switch_profile(&args.profile)?;

    let file_manager: FileManager = FileManager::new(&config, &paths);
    file_manager.update()?;

    config.save()?;

    repository.commit(&args.message)?;

    Ok(())
}
