use std::{
    collections::VecDeque,
    ffi::{OsStr, OsString},
};

use anyhow::Result;
use std::process::Command;
use subprocess::Exec;

use log::debug;
use thiserror::Error;
use twist_fs::repository::Repository;
use twist_shared::commands::ExecGitArgs;

#[derive(Error, Debug)]
enum ExecGitError {}

pub fn exec_git(args: ExecGitArgs) -> Result<()> {
    let git_current_directory_flag = OsStr::new("-C");

    let mut git_args = vec![
        git_current_directory_flag.to_os_string(),
        Repository::repo_dir(args.root_dir)
            .to_path_buf()
            .as_os_str()
            .to_os_string(),
    ];

    let mut args = VecDeque::from(args.args);

    while !args.is_empty() {
        let arg = args.pop_front().unwrap();

        if arg == git_current_directory_flag {
            match args.pop_front() {
                Some(next_arg) if next_arg.to_str().unwrap().starts_with('-') => {
                    git_args.push(next_arg)
                }
                _ => break,
            }
        } else {
            git_args.push(arg);
        }
    }

    debug!("git_args: {:?}", git_args);

    Exec::cmd("git").args(git_args.as_slice()).join()?;

    Ok(())
}
