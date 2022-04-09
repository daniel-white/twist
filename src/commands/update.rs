use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

use super::Command;

#[derive(Debug, StructOpt)]
pub struct Update {
    #[structopt(long, short = "m")]
    message: Option<String>,
}

#[derive(Error, Debug)]
enum UpdateError {
    #[error("update is not implemented")]
    NotImplemented,
}

impl Command for Update {
    fn run(&self) -> Result<()> {
        Err(UpdateError::NotImplemented)?
    }
}
