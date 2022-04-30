use anyhow::Result;
use thiserror::Error;

use crate::{Context, PushToRemoteArgs};

#[derive(Error, Debug)]
enum PushToRemoteError {
    #[error("push is not implemented")]
    NotImplementedError,
}

pub fn push_to_remote(_args: PushToRemoteArgs, _context: Context) -> Result<()> {
    Err(PushToRemoteError::NotImplementedError.into())
}
