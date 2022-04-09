use crate::fs::manager::FileManager;
use crate::fs::paths::resolve_paths;
use crate::fs::paths::ResolvedPaths;

use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

use super::Command;

#[derive(Debug, StructOpt)]
pub struct Add {
    #[structopt(long, short = "m")]
    message: Option<String>,

    #[structopt(parse(from_os_str), min_values = 1)]
    paths: Vec<PathBuf>,
}

#[derive(Error, Debug)]
enum AddError {}

impl Command for Add {
    fn run(&self) -> Result<()> {
        let mut file_manager = FileManager::new();
        let ResolvedPaths {
            file_paths,
            dir_paths,
        } = resolve_paths(&self.paths);

        file_manager.add_files(&file_paths);
        file_manager.add_dirs(&dir_paths);

        Ok(())
    }
}
