use std::path::PathBuf;

#[derive(Debug)]
pub enum Command {
    AddFiles(AddFilesArgs),
    ApplyFiles(ApplyFilesArgs),
    Init(InitArgs),
    PullFromRemote(PullFromRemoteArgs),
    PushToRemote(PushToRemoteArgs),
    RemoveFiles(RemoveFilesArgs),
    UpdateRepository(UpdateRepositoryArgs),
}

#[derive(Debug)]
pub struct AddFilesArgs {
    pub root_dir: PathBuf,
    pub profile: String,
    pub message: Option<String>,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct ApplyFilesArgs {
    pub root_dir: PathBuf,
    pub profile: String,
}

#[derive(Debug)]
pub struct InitArgs {
    pub root_dir: PathBuf,
    pub profile: String,
}

#[derive(Debug)]
pub struct PullFromRemoteArgs {
    pub root_dir: PathBuf,
    pub profile: String,
}

#[derive(Debug)]
pub struct PushToRemoteArgs {
    pub root_dir: PathBuf,
    pub profile: String,
}

#[derive(Debug)]
pub struct RemoveFilesArgs {
    pub root_dir: PathBuf,
    pub profile: String,
    pub message: Option<String>,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct UpdateRepositoryArgs {
    pub root_dir: PathBuf,
    pub profile: String,
    pub message: Option<String>,
}
