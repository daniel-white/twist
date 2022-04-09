#![feature(bool_to_option)]

mod cli;
pub mod consts;
pub mod data;
pub mod fs;
mod logging;

use crate::logging::init as init_logging;
use anyhow::Result;
use cli::parse_args;
use data::switch_profile;
use log::error;
use std::process::exit;

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
