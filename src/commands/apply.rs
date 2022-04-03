use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
pub struct ApplyOpt {}

#[derive(Error, Debug)]
enum Error {
    #[error("apply is not implemented")]
    NotImplemented,
}

pub fn run_apply(_opt: ApplyOpt, _profile: &str) -> Result<()> {
    Err(Error::NotImplemented)?
}
