use anyhow::Result;
use thiserror::Error;

use crate::RemoveFilesArgs;

#[derive(Error, Debug)]
enum RemoveFilesError {}

pub fn remove_files(_args: RemoveFilesArgs) -> Result<()> {
    // // let _repository = Repository::open(repo_dir())?;
    // // let mut file_manager = FileManager::new();
    // let ResolvedPaths {
    //     file_paths,
    //     dir_paths,
    // } = resolve_paths(&args.paths)?;

    // println!("file_paths: {:?}", file_paths);
    // println!("dir_paths: {:?}", dir_paths);

    // // file_manager.remove_files(&file_paths);
    // // file_manager.remove_dirs(&dir_paths);

    Ok(())
}
