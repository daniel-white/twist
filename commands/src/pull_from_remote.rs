use anyhow::Result;

use thiserror::Error;
use twist_shared::commands::PullFromRemoteArgs;

#[derive(Error, Debug)]
enum PullFromRemoteError {
    #[error("pull is not implemented")]
    NotImplementedError,
}

pub fn pull_from_remote(_args: PullFromRemoteArgs) -> Result<()> {
    Err(PullFromRemoteError::NotImplementedError.into())
}
