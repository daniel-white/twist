mod add_files;
mod apply_files;
mod exec_git;
mod pull_from_remote;
mod push_to_remote;
mod remove_files;
mod update_repository;

use std::{path::Path, rc::Rc};

use anyhow::Result;

use crate::{
    cli::*,
    config::*,
    files::{git::GitRepository, path::*, FileManager},
};

use add_files::*;
use apply_files::*;
use exec_git::*;
use pull_from_remote::*;
use push_to_remote::*;
use remove_files::*;
use update_repository::*;

pub fn exec_command(cli: Cli) -> Result<()> {
    let root_dir = root_dir(&cli.root_dir_override)?;
    let context = Context::new(&root_dir, &cli.profile)?;

    match Command::new(cli, context) {
        Command::ExecGit(args, context) => exec_git(args, context),
        Command::AddFiles(args, context) => add_files(args, context),
        Command::RemoveFiles(args, context) => remove_files(args, context),
        Command::ApplyFiles(args, context) => apply_files(args, context),
        Command::UpdateRepository(args, context) => update_repository(args, context),
        Command::PushToRemote(args, context) => push_to_remote(args, context),
        Command::PullFromRemote(args, context) => pull_from_remote(args, context),
        _ => Err(anyhow::anyhow!("Unsupported command")),
    }
}

pub enum Command {
    ExecGit(ExecGitArgs, Context),
    AddFiles(AddFilesArgs, Context),
    RemoveFiles(RemoveFilesArgs, Context),
    ApplyFiles(ApplyFilesArgs, Context),
    UpdateRepository(UpdateRepositoryArgs, Context),
    Init(InitArgs, Context),
    PullFromRemote(PullFromRemoteArgs, Context),
    PushToRemote(PushToRemoteArgs, Context),
}

pub struct Context {
    pub config: Rc<ConfigManager>,
    pub paths: Rc<Paths>,
    pub repo: Rc<GitRepository>,
    pub file_manager: Rc<FileManager>,
}

impl Context {
    pub fn new(root_dir: &Path, profile: &str) -> Result<Self> {
        let paths = Rc::new(Paths::new(root_dir));
        let config = Rc::new(ConfigManager::open(&paths));
        let repo = Rc::new(GitRepository::open(&paths, profile)?);
        let file_manager = Rc::new(FileManager::new(&config, &paths));

        Ok(Self {
            config,
            paths,
            repo,
            file_manager,
        })
    }
}

#[derive(Debug)]
pub struct InitArgs {}

impl Command {
    pub fn new(cli: Cli, context: Context) -> Self {
        match cli.command {
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
        }
    }
}
