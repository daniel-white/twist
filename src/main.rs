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
    run_command(cmd)

    // match  {
    //     Ok(args) => {
    //         println!("{:?}", args);
    //         Ok(())
    //         // init_logging(cli.verbose).into()
    //         //switch_profile(&cli.profile);
    //     }
    //     Err(err) => {
    //         error!("{}", err);
    //         exit(1);
    //     }
    // }
    // parse_args()
    //     .or_else(|err| {
    //         eprintln!("{}", err);
    //         exit(1);
    //     })
    //     .and_then(|args| init_logging(args.verbose).map(|_| args))
    //     .or_else(|err| {
    //         eprintln!("Logging initialization error: {}", err);
    //         exit(2);
    //     })
    //     .and_then(|args| switch_profile(args.profile.as_str()).map(|_| args))
    //     .and_then(|args| args.command.run())
    //     .or_else(|err| {
    //         error!("{}", err);
    //         exit(3);
    //     })
}
