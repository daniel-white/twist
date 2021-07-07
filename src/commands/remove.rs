use crate::fs::manager::FileManager;
use crate::fs::paths::resolve;
use crate::{DEFAULT_PROFILE, PROFILE_ENV};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE)]
    profile: String,

    #[structopt(long, short = "m")]
    message: Option<String>,

    #[structopt(parse(from_os_str))]
    paths: Vec<PathBuf>,
}

pub fn run(opt: Opt) {
    let mut file_manager = FileManager::new();
    let paths = resolve(&opt.paths);
    let file_paths = paths.iter().filter(|i| i.1.is_file());
    let dir_paths = paths.iter().filter(|i| i.1.is_dir());

    for (p, _) in file_paths {
        file_manager.remove_file(p);
    }

    for (p, _) in dir_paths {
        file_manager.remove_dir(p);
    }
}
