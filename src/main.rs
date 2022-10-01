mod cli;
mod commands;
mod config;
mod files;

use anyhow::Result;
use clap::Parser;
use log::error;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use std::process::exit;
use thiserror::Error;

use cli::Cli;
use commands::exec_command;

fn main() -> Result<()> {
    let cli = Cli::parse();

    init_logging(cli.verbose)?;

    match exec_command(cli) {
        Ok(()) => Ok(()),
        Err(err) => {
            error!("{}", err);
            exit(1);
        }
    }
}

#[derive(Error, Debug)]
enum LoggingError {
    #[error("failed to initialize logging")]
    InitFailed,
}

fn init_logging(verbose: bool) -> Result<()> {
    let log_level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        log_level,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .map_err(|_| LoggingError::InitFailed.into())
}
