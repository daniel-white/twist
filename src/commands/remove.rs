use crate::fs::manager::FileManager;
use crate::fs::paths::resolve_paths;
use crate::fs::paths::ResolvedPaths;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

use thiserror::Error;

#[derive(Error, Debug)]
enum Error {}

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long, short = "m")]
    message: Option<String>,

    #[structopt(parse(from_os_str), required = true, min_values = 1)]
    paths: Vec<PathBuf>,
}

pub fn run(opt: Opt, _profile: &str) -> Result<()> {
    let mut file_manager = FileManager::new();
    let ResolvedPaths {
        file_paths,
        dir_paths,
    } = resolve_paths(&opt.paths);

    file_manager.remove_files(&file_paths);
    file_manager.remove_dirs(&dir_paths);

    Ok(())
}
