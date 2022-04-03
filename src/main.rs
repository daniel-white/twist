#![feature(bool_to_option)]

mod commands;
pub mod data;
pub mod fs;

use anyhow::Result;
use commands::{add, apply, init, pull, push, remove, update};
use log::error;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use std::process::exit;
use structopt::StructOpt;
use thiserror::Error;

pub const DEFAULT_PROFILE: &str = "main";
pub const PROFILE_ENV: &str = "TWIST_PROFILE";

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(global = true, long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE)]
    profile: String,

    #[structopt(global = true, long, short, help = "Enable verbose logging")]
    verbose: bool,
}

#[derive(Debug, StructOpt)]
#[structopt(
    about = "A tool for managing your dotfiles with a twist",
    rename_all = "kebab-case"
)]
enum Command {
    #[structopt(about = "Adds the given files and directories to the dotfiles repository")]
    Add(add::AddOpt),
    #[structopt(about = "Applies the dotfiles to the current system")]
    Apply(apply::ApplyOpt),
    #[structopt(about = "Initializes the dotfiles repository")]
    Init(init::Opt),
    #[structopt(about = "Pulls the dotfiles from the remote repository")]
    Pull(pull::Opt),
    #[structopt(about = "Pushes the dotfiles to the remote repository")]
    Push(push::Opt),
    #[structopt(
        about = "Removes the given files and directories from the dotfiles repository",
        alias = "rm"
    )]
    Remove(remove::Opt),
    #[structopt(about = "Updates the dotfiles repository from the current system")]
    Update(update::Opt),
}

#[derive(Error, Debug)]
enum Error {
    #[error("failed to initialize logger")]
    LoggerInitFailed,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    TermLogger::init(
        opt.verbose
            .then_some(LevelFilter::Debug)
            .unwrap_or(LevelFilter::Info),
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or(Err(Error::LoggerInitFailed))?;

    let profile = &opt.profile;

    match opt.command {
        Command::Add(opt) => add::run_add(opt, &profile),
        Command::Apply(opt) => apply::run_apply(opt, &profile),
        Command::Init(opt) => init::run(opt, &profile),
        Command::Pull(opt) => pull::run(opt, &profile),
        Command::Push(opt) => push::run(opt, &profile),
        Command::Remove(opt) => remove::run(opt, &profile),
        Command::Update(opt) => update::run_update(opt, &profile),
    }
    .map_err(|err| {
        error!("{}", err);
        exit(1);
    })
}
