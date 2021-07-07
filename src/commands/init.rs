use crate::fs::ensure_dir;
use crate::fs::paths::{repo_dir, root_dir};
use git2::Repository;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {}

pub fn run(_opt: Opt) {
    ensure_dir(root_dir());

    let sig = git2::Signature::new("Example", "Example", &git2::Time::new(0, 0)).unwrap();
    let repo = Repository::init(repo_dir()).unwrap();
    let mut index = repo.index().unwrap();
    let tree_id = index.write_tree().unwrap();
    let tree = repo.find_tree(tree_id).unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "Initial Commit", &tree, &[])
        .unwrap();
}
