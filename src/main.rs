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

use cli::{Cli, CliCommand};
use commands::*;
use files::path::root_dir;

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logging(cli.verbose)?;

    match run_command(cli.try_into()?) {
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

impl TryInto<Command> for Cli {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<Command> {
        let root_dir = root_dir(self.root_dir_override)?;
        let context = Context::new(&root_dir, &self.profile)?;

        let command = match self.command {
            CliCommand::ExecGit(args) => Command::ExecGit(ExecGitArgs { args: args.args }, context),
            CliCommand::AddFiles(args) => Command::AddFiles(
                AddFilesArgs {
                    message: args.message,
                    paths: args.paths,
                },
                context,
            ),
            CliCommand::RemoveFiles(args) => Command::RemoveFiles(
                RemoveFilesArgs {
                    message: args.message,
                    paths: args.paths,
                },
                context,
            ),
            CliCommand::ApplyFiles(_args) => Command::ApplyFiles(ApplyFilesArgs {}, context),
            CliCommand::UpdateRepository(args) => Command::UpdateRepository(
                UpdateRepositoryArgs {
                    message: args.message,
                },
                context,
            ),
            CliCommand::Init(_args) => Command::Init(InitArgs {}, context),
            CliCommand::PullFromRemote(_args) => {
                Command::PullFromRemote(PullFromRemoteArgs {}, context)
            }
            CliCommand::PushToRemote(_args) => Command::PushToRemote(PushToRemoteArgs {}, context),
        };

        Ok(command)
    }
}
