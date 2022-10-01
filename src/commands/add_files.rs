use anyhow::Result;
use thiserror::Error;

use crate::{AddFilesArgs, Context};

#[derive(Error, Debug)]
enum AddFilesError {}

pub fn add_files(args: AddFilesArgs, context: Context) -> Result<()> {
    context.file_manager.add(&args.paths)?;
    context.config.save()?;
    context.repo.commit(&args.message)?;

    Ok(())
}
