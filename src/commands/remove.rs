use super::Command;
use crate::fs::manager::FileManager;
use crate::fs::paths::resolve_paths;
use crate::fs::paths::ResolvedPaths;
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;
use thiserror::Error;

#[derive(Debug, StructOpt)]
pub struct Remove {
    #[structopt(long, short = "m")]
    message: Option<String>,

    #[structopt(parse(from_os_str), required = true, min_values = 1)]
    paths: Vec<PathBuf>,
}

#[derive(Error, Debug)]
enum RemoveError {}

impl Command for Remove {
    fn run(&self) -> Result<()> {
        let mut file_manager = FileManager::new();
        let ResolvedPaths {
            file_paths,
            dir_paths,
        } = resolve_paths(&self.paths);

        file_manager.remove_files(&file_paths);
        file_manager.remove_dirs(&dir_paths);

        Ok(())
    }
}
