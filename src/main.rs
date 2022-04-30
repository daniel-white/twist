use anyhow::Result;
use twist_cli::init as init_cli;

use log::error;
use std::process::exit;
use twist_commands::run_command;

fn main() -> Result<()> {
    let cmd = init_cli()?;
    match run_command(cmd) {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }
}
