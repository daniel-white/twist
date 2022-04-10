use anyhow::Result;

use thiserror::Error;
use twist_shared::commands::ApplyFilesArgs;

#[derive(Error, Debug)]
enum ApplyFilesError {
    #[error("apply is not implemented")]
    NotImplementedError,
}

pub fn apply_files(args: ApplyFilesArgs) -> Result<()> {
    Err(ApplyFilesError::NotImplementedError)?
}
