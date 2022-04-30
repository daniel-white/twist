use anyhow::Result;
use thiserror::Error;

use crate::{Context, RemoveFilesArgs};

#[derive(Error, Debug)]
enum RemoveFilesError {}

pub fn remove_files(args: RemoveFilesArgs, context: Context) -> Result<()> {
    context.file_manager.remove(&args.paths)?;
    context.config.save()?;
    context.repo.commit(&args.message)?;

    Ok(())
}
