use clap::{Args, Parser, Subcommand};
use std::ffi::OsString;
use std::path::PathBuf;

const DEFAULT_PROFILE: &str = "main";
const PROFILE_ENV: &str = "TWIST_PROFILE";
const ROOT_DIR_ENV: &str = "TWIST_ROOTDIR";

const DEFAULT_COMMIT_MESSAGE_FOR_ADD: &str = "Adding new dotfiles";
const DEFAULT_COMMIT_MESSAGE_FOR_UPDATE: &str = "Updating dotfiles";
const DEFAULT_COMMIT_MESSAGE_FOR_REMOVE: &str = "Removing dotfiles";

#[derive(Debug, Parser)]
#[command(
    about = "A tool for managing your dotfiles with a twist",
    allow_missing_positional = true
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: CliCommand,

    #[arg(long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE, help = "Set the profile used")]
    pub profile: String,

    #[arg(global = true, long = "root-dir", env = ROOT_DIR_ENV, help = "Override the default root directory")]
    pub root_dir_override: Option<PathBuf>,

    #[arg(long, short, help = "Enable verbose logging")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum CliCommand {
    #[command(about = "Executes git in the root directory", name = "git")]
    ExecGit(ExecGitCliArgs),

    #[command(
        about = "Adds the given files and directories to the dotfiles repository",
        name = "add"
    )]
    AddFiles(AddFilesCliArgs),

    #[command(
        about = "Removes the given files and directories from the dotfiles repository",
        name = "remove",
        alias = "rm"
    )]
    RemoveFiles(RemoveFilesCliArgs),

    #[command(about = "Applies the dotfiles to the current system", name = "apply")]
    ApplyFiles(ApplyFilesCliArgs),

    #[command(
        about = "Updates the dotfiles repository from the current system",
        name = "update"
    )]
    UpdateRepository(UpdateRepositoryCliArgs),

    #[command(about = "Initializes the dotfiles repository", bin_name = "twist")]
    Init(InitCliArgs),

    #[command(about = "Pulls the dotfiles from the remote repository", name = "pull")]
    PullFromRemote(PullFromRemoteCliArgs),

    #[command(about = "Pushes the dotfiles to the remote repository", name = "push")]
    PushToRemote(PushToRemoteCliArgs),
}

#[derive(Debug, Args)]
pub struct ExecGitCliArgs {
    #[arg(long, short = 'm')]
    pub message: Option<String>,

    #[arg(required = true)]
    pub args: Vec<OsString>,
}

#[derive(Debug, Args)]
pub struct AddFilesCliArgs {
    #[arg(long, short = 'm', default_value = DEFAULT_COMMIT_MESSAGE_FOR_ADD)]
    pub message: String,

    #[arg(required = true)]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
pub struct ApplyFilesCliArgs {}

#[derive(Debug, Args)]
pub struct InitCliArgs {}

#[derive(Debug, Args)]
pub struct PullFromRemoteCliArgs {}

#[derive(Debug, Args)]
pub struct PushToRemoteCliArgs {}

#[derive(Debug, Args)]
pub struct RemoveFilesCliArgs {
    #[arg(long, short = 'm', default_value = DEFAULT_COMMIT_MESSAGE_FOR_REMOVE)]
    pub message: String,

    #[arg(required = true)]
    pub paths: Vec<PathBuf>,
}

#[derive(Debug, Args)]
pub struct UpdateRepositoryCliArgs {
    #[arg(long, short = 'm', default_value = DEFAULT_COMMIT_MESSAGE_FOR_UPDATE)]
    pub message: String,
}
