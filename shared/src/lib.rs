#![feature(bool_to_option)]

pub mod commands;

pub const DEFAULT_PROFILE: &str = "main";
pub const PROFILE_ENV: &str = "TWIST_PROFILE";
pub const ROOT_DIR_ENV: &str = "TWIST_ROOTDIR";

pub const ROOT_DIR_NAME: &str = ".twist";
pub const DOTFILES_DIR_NAME: &str = "dotfiles";
