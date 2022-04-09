mod commands;

use anyhow::Result;
use structopt::StructOpt;
use thiserror::Error;

use commands::CommandVerb;

use crate::consts::{DEFAULT_PROFILE, PROFILE_ENV};

#[derive(Debug, StructOpt)]
#[structopt(about = "A tool for managing your dotfiles with a twist")]
pub struct CommandArgs {
    #[structopt(subcommand)]
    pub command: CommandVerb,

    #[structopt(global = true, long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE)]
    pub profile: String,

    #[structopt(global = true, long, short, help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(Error, Debug)]
enum CliErrors {}

pub fn parse_args() -> Result<CommandArgs> {
    match CommandArgs::from_args_safe() {
        Ok(args) => Ok(args),
        Err(err) => Err(err.into()),
    }
}
