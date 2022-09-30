#[macro_use]
extern crate shadow_rs;

mod logging;
mod path;

use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, IntoApp, Parser, Subcommand};

use logging::init as init_logging;
use path::root_dir;
use twist_commands::{
    AddFilesArgs, ApplyFilesArgs, Command, Context, ExecGitArgs, InitArgs, PullFromRemoteArgs,
    PushToRemoteArgs, RemoveFilesArgs, UpdateRepositoryArgs,
};
use twist_common::{
    DEFAULT_COMMIT_MESSAGE_FOR_ADD, DEFAULT_COMMIT_MESSAGE_FOR_REMOVE,
    DEFAULT_COMMIT_MESSAGE_FOR_UPDATE, DEFAULT_PROFILE, PROFILE_ENV, ROOT_DIR_ENV,
};

shadow!(shadow);

#[derive(Debug, Parser)]
#[clap(
    about = "A tool for managing your dotfiles with a twist",
    version=shadow::PKG_VERSION,
    long_version=shadow::CLAP_LONG_VERSION,
    allow_missing_positional = true
)]
struct Cli {
    #[clap(subcommand)]
    pub command: CliCommand,

    #[clap(long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE, help = "Set the profile used")]
    pub profile: String,

    #[clap(global = true, long = "root-dir", env = ROOT_DIR_ENV, help = "Override the default root directory")]
    pub root_dir_override: Option<PathBuf>,

    #[clap(long, short, help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    #[clap(about = "Executes git in the root directory", name = "git")]
    ExecGit(ExecGitCliArgs),

    #[clap(
        about = "Adds the given files and directories to the dotfiles repository",
        name = "add"
    )]
    AddFiles(AddFilesCliArgs),

    #[clap(
        about = "Removes the given files and directories from the dotfiles repository",
        name = "remove",
        alias = "rm"
    )]
    RemoveFiles(RemoveFilesCliArgs),

    #[clap(about = "Applies the dotfiles to the current system", name = "apply")]
    ApplyFiles(ApplyFilesCliArgs),

    #[clap(
        about = "Updates the dotfiles repository from the current system",
        name = "update"
    )]
    UpdateRepository(UpdateRepositoryCliArgs),

    #[clap(about = "Initializes the dotfiles repository", bin_name = "twist")]
    Init(InitCliArgs),

    #[clap(about = "Pulls the dotfiles from the remote repository", name = "pull")]
    PullFromRemote(PullFromRemoteCliArgs),

    #[clap(about = "Pushes the dotfiles to the remote repository", name = "push")]
    PushToRemote(PushToRemoteCliArgs),
}

impl Cli {
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

pub fn init() -> Result<Command> {
    match Cli::try_parse() {
        Ok(cli) => {
            init_logging(cli.verbose)?;
            cli.try_into()
        }
        Err(err) => Err(err.into()),
    }
}

pub fn cli() -> clap::Command<'static> {
    Cli::command()
}

#[derive(Debug, Args)]
struct ExecGitCliArgs {
    #[clap(long, short = 'm')]
    message: Option<String>,

    #[clap()]
    args: Vec<OsString>,
}

#[derive(Debug, Args)]
struct AddFilesCliArgs {
    #[clap(long, short = 'm', default_value = DEFAULT_COMMIT_MESSAGE_FOR_ADD)]
    message: String,

    #[clap(min_values = 1)]
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
    #[clap(long, short = 'm', default_value = DEFAULT_COMMIT_MESSAGE_FOR_REMOVE)]
    message: String,

    #[clap(min_values = 1)]
    paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
struct UpdateRepositoryCliArgs {
    #[clap(long, short = 'm', default_value = DEFAULT_COMMIT_MESSAGE_FOR_UPDATE)]
    message: String,
}
