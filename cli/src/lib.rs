#![feature(bool_to_option)]

mod logging;

use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, IntoApp, Parser, Subcommand};

use logging::init as init_logging;
use twist_shared::commands::{
    AddFilesArgs, ApplyFilesArgs, Command, InitArgs, PullFromRemoteArgs, PushToRemoteArgs,
    RemoveFilesArgs, UpdateRepositoryArgs,
};
use twist_shared::{DEFAULT_PROFILE, PROFILE_ENV};

#[derive(Debug, Parser)]
#[clap(about = "A tool for managing your dotfiles with a twist")]
struct Cli {
    #[clap(subcommand)]
    pub command: CliCommand,

    #[clap(global = true, long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE)]
    pub profile: String,

    #[clap(global = true, long, short, help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    #[clap(
        about = "Adds the given files and directories to the dotfiles repository",
        name = "add"
    )]
    AddFiles(AddFilesCliArgs),

    #[clap(about = "Applies the dotfiles to the current system", name = "apply")]
    ApplyFiles(ApplyFilesCliArgs),

    #[clap(about = "Initializes the dotfiles repository")]
    Init(InitCliArgs),

    #[clap(about = "Pulls the dotfiles from the remote repository", name = "pull")]
    PullFromRemote(PullFromRemoteCliArgs),

    #[clap(about = "Pushes the dotfiles to the remote repository", name = "push")]
    PushToRemote(PushToRemoteCliArgs),

    #[clap(
        about = "Removes the given files and directories from the dotfiles repository",
        name = "remove",
        alias = "rm"
    )]
    RemoveFiles(RemoveFilesCliArgs),

    #[clap(
        about = "Updates the dotfiles repository from the current system",
        name = "update"
    )]
    UpdateRepository(UpdateRepositoryCliArgs),
}

impl Into<Command> for Cli {
    fn into(self) -> Command {
        match self.command {
            CliCommand::AddFiles(args) => Command::AddFiles(AddFilesArgs {
                profile: self.profile,
                message: args.message,
                paths: args.paths,
            }),
            CliCommand::ApplyFiles(args) => Command::ApplyFiles(ApplyFilesArgs {
                profile: self.profile,
            }),
            CliCommand::Init(args) => Command::Init(InitArgs {
                profile: self.profile,
            }),
            CliCommand::PullFromRemote(args) => Command::PullFromRemote(PullFromRemoteArgs {
                profile: self.profile,
            }),
            CliCommand::PushToRemote(args) => Command::PushToRemote(PushToRemoteArgs {
                profile: self.profile,
            }),
            CliCommand::RemoveFiles(args) => Command::RemoveFiles(RemoveFilesArgs {
                profile: self.profile,
                message: args.message,
                paths: args.paths,
            }),
            CliCommand::UpdateRepository(args) => Command::UpdateRepository(UpdateRepositoryArgs {
                profile: self.profile,
                message: args.message,
            }),
        }
    }
}

pub fn init() -> Result<Command> {
    match Cli::try_parse() {
        Ok(args) => {
            init_logging(args.verbose)?;
            Ok(args.into())
        }
        Err(err) => Err(err.into()),
    }
}

pub fn into_command() -> clap::Command<'static> {
    Cli::command()
}

#[derive(Debug, Args)]
struct AddFilesCliArgs {
    #[clap(long, short = 'm')]
    message: Option<String>,

    #[clap(parse(from_os_str), min_values = 1)]
    paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct ApplyFilesCliArgs {}

#[derive(Debug, Args)]
struct InitCliArgs {}

#[derive(Debug, Args)]
struct PullFromRemoteCliArgs {}

#[derive(Debug, Args)]
struct PushToRemoteCliArgs {}

#[derive(Debug, Args)]
struct RemoveFilesCliArgs {
    #[clap(long, short = 'm')]
    message: Option<String>,

    #[clap(parse(from_os_str), min_values = 1)]
    paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct UpdateRepositoryCliArgs {
    #[clap(long, short = 'm')]
    message: Option<String>,
}
