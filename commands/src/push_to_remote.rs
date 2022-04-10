use anyhow::Result;

use thiserror::Error;
use twist_shared::commands::PushToRemoteArgs;

#[derive(Error, Debug)]
enum PushToRemoteError {
    #[error("push is not implemented")]
    NotImplementedError,
}

pub fn push_to_remote(args: PushToRemoteArgs) -> Result<()> {
    Err(PushToRemoteError::NotImplementedError)?
}
