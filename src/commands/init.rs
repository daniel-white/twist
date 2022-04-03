use crate::fs::ensure_dir;
use crate::fs::paths::{repo_dir, root_dir};

use anyhow::Result;
use git2::Repository;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {}

pub fn run(_opt: Opt, _profile: &str) -> Result<()> {
    ensure_dir(root_dir());

    let sig = git2::Signature::new("Example", "Example", &git2::Time::new(0, 0))?;
    let repo_dir = repo_dir();
    let repo =
        Repository::open(repo_dir.clone()).or_else(|_| Repository::init(repo_dir.clone()))?;
    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;
    repo.commit(Some("HEAD"), &sig, &sig, "Initial Commit", &tree, &[])?;

    Ok(())
}
