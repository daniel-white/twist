use anyhow::Result;

mod cli;
mod logging;
mod path;

use clap::Parser;
use cli::{Cli, CliCommand};
use log::error;
use logging::init as init_logging;
use path::root_dir;
use std::process::exit;
use twist_commands::*;

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
