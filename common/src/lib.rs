pub mod config;
pub mod files;
pub mod path;

pub const DEFAULT_PROFILE: &str = "main";
pub const PROFILE_ENV: &str = "TWIST_PROFILE";
pub const ROOT_DIR_ENV: &str = "TWIST_ROOTDIR";

pub const ROOT_DIR_NAME: &str = ".twist";

pub const DEFAULT_COMMIT_MESSAGE_FOR_ADD: &str = "Adding new dotfiles";
pub const DEFAULT_COMMIT_MESSAGE_FOR_UPDATE: &str = "Updating dotfiles";
pub const DEFAULT_COMMIT_MESSAGE_FOR_REMOVE: &str = "Removing dotfiles";
pub const DEFAULT_COMMITTER_NAME: &str = "Twist";
pub const DEFAULT_COMMITTER_EMAIL: &str = "twist@example.com";
