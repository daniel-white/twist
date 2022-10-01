use std::path::PathBuf;

use anyhow::Result;
use thiserror::Error;

use super::Context;

#[derive(Debug)]
pub struct AddFilesArgs {
    pub message: String,
    pub paths: Vec<PathBuf>,
}

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs, context: Context) -> Result<()> {
    context.file_manager.add(&args.paths)?;
    context.config.save()?;
    context.repo.commit(&args.message)?;

    Ok(())
}
