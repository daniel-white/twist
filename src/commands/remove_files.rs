use anyhow::Result;
use std::path::PathBuf;

use super::Context;

#[derive(Debug)]
pub struct RemoveFilesArgs {
    pub message: String,
    pub paths: Vec<PathBuf>,
}

pub fn remove_files(args: RemoveFilesArgs, context: Context) -> Result<()> {
    context.file_manager.remove(&args.paths)?;
    context.config.save()?;
    context.repo.commit(&args.message)?;

    Ok(())
}
