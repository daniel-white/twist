use anyhow::Result;
use thiserror::Error;

use super::Context;

#[derive(Debug)]
pub struct PullFromRemoteArgs {}

#[derive(Error, Debug)]
enum PullFromRemoteError {
    #[error("pull is not implemented")]
    NotImplementedError,
}

pub fn pull_from_remote(_args: PullFromRemoteArgs, _context: Context) -> Result<()> {
    Err(PullFromRemoteError::NotImplementedError.into())
}
