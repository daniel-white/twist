use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Error, Debug)]
enum Error {
    #[error("update is not implemented")]
    NotImplemented,
}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long, short = "m")]
    message: Option<String>,
}

pub fn run_update(_opt: Opt, _profile: &str) -> Result<()> {
    Err(Error::NotImplemented)?
}
