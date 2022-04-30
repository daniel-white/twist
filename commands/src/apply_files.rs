use anyhow::Result;
use thiserror::Error;

use crate::{ApplyFilesArgs, Context};

#[derive(Error, Debug)]
enum ApplyFilesError {
    #[error("apply is not implemented")]
    NotImplementedError,
}

pub fn apply_files(_args: ApplyFilesArgs, _context: Context) -> Result<()> {
    Err(ApplyFilesError::NotImplementedError.into())
}
