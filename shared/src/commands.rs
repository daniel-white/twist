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
    pub profile: String,
    pub message: Option<String>,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct ApplyFilesArgs {
    pub profile: String,
}

#[derive(Debug)]
pub struct InitArgs {
    pub profile: String,
}

#[derive(Debug)]
pub struct PullFromRemoteArgs {
    pub profile: String,
}

#[derive(Debug)]
pub struct PushToRemoteArgs {
    pub profile: String,
}

#[derive(Debug)]
pub struct RemoveFilesArgs {
    pub profile: String,
    pub message: Option<String>,
    pub paths: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct UpdateRepositoryArgs {
    pub profile: String,
    pub message: Option<String>,
}
