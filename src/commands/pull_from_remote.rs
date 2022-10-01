use anyhow::Result;
use thiserror::Error;

use crate::{Context, PullFromRemoteArgs};

#[derive(Error, Debug)]
enum PullFromRemoteError {
    #[error("pull is not implemented")]
    NotImplementedError,
}

pub fn pull_from_remote(_args: PullFromRemoteArgs, _context: Context) -> Result<()> {
    Err(PullFromRemoteError::NotImplementedError.into())
}
