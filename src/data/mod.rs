pub mod metadata;

use anyhow::Result;
use log::info;
use thiserror::Error;

use crate::DEFAULT_PROFILE;

#[derive(Debug, Error)]
pub enum ProfileError {
    #[error("metadata is not implemented")]
    NotImplemented,
}

pub fn switch_profile(profile: &str) -> Result<()> {
    if profile == "xxx" {
        return Err(ProfileError::NotImplemented.into());
    }
    if profile != DEFAULT_PROFILE {
        info!("Switching to profile '{}'", &profile);
    }
    Ok(())
}
