#[macro_use]
extern crate shadow_rs;

use anyhow::Result;

mod cli;
mod logging;

use clap::Parser;
use cli::Cli;
use log::error;
use logging::init as init_logging;
use std::process::exit;
use twist_commands::{run_command, Command};

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.verbose)?;

    let command: Command = cli.try_into()?;

    match run_command(command) {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }
}
