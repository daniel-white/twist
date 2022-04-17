use std::path::PathBuf;

pub mod commands;
pub mod config;

pub const DEFAULT_PROFILE: &str = "main";
pub const PROFILE_ENV: &str = "TWIST_PROFILE";
pub const ROOT_DIR_ENV: &str = "TWIST_ROOTDIR";

pub const ROOT_DIR_NAME: &str = ".twist";
pub const FILES_DIR_NAME: &str = "dotfiles";

#[derive(Debug, PartialEq)]
pub struct FilePathInfo {
    pub full_src_path: PathBuf,
    pub src_path: PathBuf,
    pub full_repo_path: PathBuf,
    pub repo_path: PathBuf,
    pub config_repo_path: PathBuf,
}
