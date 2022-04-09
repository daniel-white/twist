use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

use super::Command;

#[derive(Debug, StructOpt)]
pub struct Apply {}

#[derive(Error, Debug)]
enum ApplyError {
    #[error("apply is not implemented")]
    NotImplemented,
}

impl Command for Apply {
    fn run(&self) -> Result<()> {
        Err(ApplyError::NotImplemented)?
    }
}
