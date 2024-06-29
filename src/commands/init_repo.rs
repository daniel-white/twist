use std::fs::create_dir_all;

use anyhow::Result;
use git2::Repository;
use thiserror::Error;

use super::Context;

#[derive(Debug)]
pub struct InitRepoArgs {}

#[derive(Error, Debug)]
enum InitError {
    #[error("unable to create repository directory")]
    UnableToCreateDirectory,
}

pub fn init_repo(_args: InitRepoArgs, context: Context) -> Result<()> {
    let repo_dir = &context.paths.files_dir;

    create_dir_all(repo_dir).map_err(|_| InitError::UnableToCreateDirectory)?;

    let sig = git2::Signature::new("Example", "Example", &git2::Time::new(0, 0))?;
    let repo = Repository::open(repo_dir).or_else(|_| Repository::init(repo_dir))?;
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    repo.commit(Some("HEAD"), &sig, &sig, "Initial Commit", &tree, &[])?;

    Ok(())
}
