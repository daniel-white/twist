use anyhow::Result;

use thiserror::Error;
use twist_common::commands::ApplyFilesArgs;

#[derive(Error, Debug)]
enum ApplyFilesError {
    #[error("apply is not implemented")]
    NotImplementedError,
}

pub fn apply_files(_args: ApplyFilesArgs) -> Result<()> {
    Err(ApplyFilesError::NotImplementedError.into())
}
