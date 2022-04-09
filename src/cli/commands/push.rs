use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

use super::Command;

#[derive(Debug, StructOpt)]
pub struct Push {}

#[derive(Error, Debug)]
enum PushError {
    #[error("push is not implemented")]
    NotImplemented,
}

impl Command for Push {
    fn run(&self) -> Result<()> {
        Err(PushError::NotImplemented)?
    }
}
