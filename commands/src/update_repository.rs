use anyhow::Result;
use thiserror::Error;

use crate::{Context, UpdateRepositoryArgs};

#[derive(Error, Debug)]
enum UpdateRepositoryError {}

pub fn update_repository(args: UpdateRepositoryArgs, context: Context) -> Result<()> {
    context.file_manager.update()?;
    context.config.save()?;
    context.repo.commit(&args.message)?;

    Ok(())
}
