use anyhow::Result;
use thiserror::Error;

use crate::UpdateRepositoryArgs;

#[derive(Error, Debug)]
enum UpdateRepositoryError {
    #[error("update is not implemented")]
    NotImplementedError,
}

pub fn update_repository(_args: UpdateRepositoryArgs) -> Result<()> {
    Err(UpdateRepositoryError::NotImplementedError.into())
}
