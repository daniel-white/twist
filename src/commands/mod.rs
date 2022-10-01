mod add_files;
mod apply_files;
mod exec_git;
mod pull_from_remote;
mod push_to_remote;
mod remove_files;
mod update_repository;

use anyhow::Result;

use add_files::add_files;
use apply_files::apply_files;
use exec_git::exec_git;
use pull_from_remote::pull_from_remote;
use push_to_remote::push_to_remote;
use remove_files::remove_files;
use twist_common::{
    config::ConfigManager,
    files::{git::GitRepository, FileManager},
    path::Paths,
};
use update_repository::update_repository;

pub fn run_command(command: Command) -> Result<()> {
    match command {
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

use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    rc::Rc,
};

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
pub struct ExecGitArgs {
    pub args: Vec<OsString>,
}

#[derive(Debug)]
pub struct AddFilesArgs {
    pub message: String,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct ApplyFilesArgs {}

#[derive(Debug)]
pub struct InitArgs {}

#[derive(Debug)]
pub struct PullFromRemoteArgs {}

#[derive(Debug)]
pub struct PushToRemoteArgs {}

#[derive(Debug)]
pub struct RemoveFilesArgs {
    pub message: String,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct UpdateRepositoryArgs {
    pub message: String,
}
