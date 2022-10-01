use anyhow::Result;
use thiserror::Error;

use super::Context;

#[derive(Debug)]
pub struct UpdateRepositoryArgs {
    pub message: String,
}

#[derive(Error, Debug)]
enum UpdateRepositoryError {}

pub fn update_repository(args: UpdateRepositoryArgs, context: Context) -> Result<()> {
    context.file_manager.update()?;
    context.config.save()?;
    context.repo.commit(&args.message)?;

    Ok(())
}
