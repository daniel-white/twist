use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("push is not implemented")]
    NotImplemented,
}

#[derive(Debug, StructOpt)]
pub struct Opt {}

pub fn run(_opt: Opt, _profile: &str) -> Result<()> {
    Err(Error::NotImplemented)?
}
