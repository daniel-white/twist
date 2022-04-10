#![feature(bool_to_option)]

// pub mod data;
// pub mod fs;

use anyhow::Result;
use twist_cli::init as init_cli;

// use data::switch_profile;

use log::{debug, error};
use std::process::exit;
use twist_commands::run_command;

fn main() -> Result<()> {
    let cmd = init_cli()?;
    debug!("Command: {:?}", cmd);
    match run_command(cmd) {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }
}
