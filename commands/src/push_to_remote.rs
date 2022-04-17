use anyhow::Result;

use thiserror::Error;
use twist_common::commands::PushToRemoteArgs;

#[derive(Error, Debug)]
enum PushToRemoteError {
    #[error("push is not implemented")]
    NotImplementedError,
}

pub fn push_to_remote(_args: PushToRemoteArgs) -> Result<()> {
    Err(PushToRemoteError::NotImplementedError.into())
}
