#![feature(bool_to_option)]

mod commands;
pub mod data;
pub mod fs;
mod logging;

use crate::logging::init as init_logging;
use anyhow::Result;
use commands::CommandVerb;
use data::switch_profile;
use log::error;
use std::process::exit;
use structopt::StructOpt;
use strum_macros::Display;
use thiserror::Error;

pub const DEFAULT_PROFILE: &str = "main";
pub const PROFILE_ENV: &str = "TWIST_PROFILE";

#[derive(Debug, StructOpt)]
#[structopt(about = "A tool for managing your dotfiles with a twist")]
struct CommandArgs {
    #[structopt(subcommand)]
    command: CommandVerb,

    #[structopt(global = true, long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE)]
    profile: String,

    #[structopt(global = true, long, short, help = "Enable verbose logging")]
    verbose: bool,
}

#[derive(Error, Debug)]
enum MainError {}

fn parse_args() -> Result<CommandArgs> {
    match CommandArgs::from_args_safe() {
        Ok(args) => Ok(args),
        Err(err) => Err(err.into()),
    }
}

fn main() -> Result<()> {
    parse_args()
        .or_else(|err| {
            eprintln!("{}", err);
            exit(1);
        })
        .and_then(|args| init_logging(args.verbose).map(|_| args))
        .or_else(|err| {
            eprintln!("Logging initialization error: {}", err);
            exit(2);
        })
        .and_then(|args| switch_profile(args.profile.as_str()).map(|_| args))
        .and_then(|args| args.command.run())
        .or_else(|err| {
            error!("{}", err);
            exit(3);
        })
}
