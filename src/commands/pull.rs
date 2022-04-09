use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

use super::Command;

#[derive(Debug, StructOpt)]
pub struct Pull {}

#[derive(Error, Debug)]
enum PullError {
    #[error("pull is not implemented")]
    NotImplemented,
}

impl Command for Pull {
    fn run(&self) -> Result<()> {
        Err(PullError::NotImplemented)?
    }
}
