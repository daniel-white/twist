mod add_files;
mod apply_files;
mod exec_git;
mod pull_from_remote;
mod push_to_remote;
mod remove_files;
mod update_repository;

use anyhow::Result;
use twist_common::commands::Command;

use add_files::add_files;
use apply_files::apply_files;
use exec_git::exec_git;
use pull_from_remote::pull_from_remote;
use push_to_remote::push_to_remote;
use remove_files::remove_files;
use update_repository::update_repository;

pub fn run_command(command: Command) -> Result<()> {
    match command {
        Command::ExecGit(args) => exec_git(args),
        Command::AddFiles(args) => add_files(args),
        Command::RemoveFiles(args) => remove_files(args),
        Command::ApplyFiles(args) => apply_files(args),
        Command::UpdateRepository(args) => update_repository(args),
        Command::PushToRemote(args) => push_to_remote(args),
        Command::PullFromRemote(args) => pull_from_remote(args),
        _ => Err(anyhow::anyhow!("Unsupported command")),
    }
}
