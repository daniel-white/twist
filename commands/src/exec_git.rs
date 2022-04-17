use std::{
    env,
    ffi::{OsStr, OsString},
    path::Path,
    vec::IntoIter,
};

use anyhow::Result;

use log::{debug, warn};
use subprocess::Exec;
use thiserror::Error;
use twist_common::{commands::ExecGitArgs, path::Paths};

#[derive(Error, Debug)]
enum ExecGitError {
    #[error("git returned non-zero status code")]
    GitError,

    #[error("failed to exec git: {0}")]
    GitExecError(anyhow::Error),
}

const GIT_CURRENT_DIRECTORY_FLAG: &str = "-C";
const GIT_WORK_TREE_FLAG: &str = "--work-tree";
const GIT_DIR_FLAG: &str = "--git-dir";
const GIT_EXEC_PATH_ENV_VAR: &str = "GIT_EXEC_PATH";

pub fn exec_git(args: ExecGitArgs) -> Result<()> {
    let paths = Paths::new(args.root_dir);
    let git_current_directory_flag = OsStr::new(GIT_CURRENT_DIRECTORY_FLAG);

    let git_path = env::var(GIT_EXEC_PATH_ENV_VAR)
        .ok()
        .or_else(|| Some(String::default()))
        .map(|p| Path::new(&p).join("git"))
        .unwrap();

    let mut git_args = vec![
        git_current_directory_flag.to_os_string(),
        OsString::from(&paths.files_dir),
    ];

    let mut safe_git_args: Vec<_> = SafeExecGitArgs::from(args.args).collect();
    git_args.append(&mut safe_git_args);

    debug!("executing {:?} with args: {:?}", git_path, git_args);

    match Exec::cmd(git_path).args(&git_args).join() {
        Ok(exit_status) if exit_status.success() => {
            debug!("successfully executed git command");
            Ok(())
        }
        Ok(_) => Err(ExecGitError::GitError.into()),
        Err(err) => Err(ExecGitError::GitExecError(err.into()).into()),
    }
}

struct SafeExecGitArgs(IntoIter<OsString>);

impl SafeExecGitArgs {
    fn is_safe_arg(arg: &OsStr) -> bool {
        !Self::is_blocked_flag(arg)
    }

    fn is_blocked_flag(arg: &OsStr) -> bool {
        arg == GIT_CURRENT_DIRECTORY_FLAG || arg == GIT_WORK_TREE_FLAG || arg == GIT_DIR_FLAG
    }
}

impl From<Vec<OsString>> for SafeExecGitArgs {
    fn from(args: Vec<OsString>) -> Self {
        Self(args.into_iter())
    }
}

impl Iterator for SafeExecGitArgs {
    type Item = OsString;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.next() {
            Some(arg) if Self::is_safe_arg(&arg) => Some(arg),
            Some(arg) if Self::is_blocked_flag(&arg) => {
                warn!("git command arguments must not contain the {:?} flag", arg);

                self.0.next(); // consume the next argument
                self.next() // then get the next safe argument
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_args() {
        let args = vec![
            OsString::from("-C"),
            OsString::from("c-arg"),
            OsString::from("--git-dir"),
            OsString::from("git-dir"),
            OsString::from("--work-tree"),
            OsString::from("work-tree"),
            OsString::from("--bare"),
            OsString::from("--no-checkout"),
            OsString::from("--"),
            OsString::from("add"),
            OsString::from("file1"),
            OsString::from("file2"),
        ];

        let safe_args: Vec<_> = SafeExecGitArgs::from(args).collect();

        assert_eq!(
            safe_args,
            vec![
                OsString::from("--bare"),
                OsString::from("--no-checkout"),
                OsString::from("--"),
                OsString::from("add"),
                OsString::from("file1"),
                OsString::from("file2"),
            ]
        );
    }
}
