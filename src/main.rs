mod commands;
pub mod data;
pub mod fs;

use commands::{add, apply, init, pull, push, remove, update};
use simplelog::*;
use structopt::StructOpt;

pub const DEFAULT_PROFILE: &str = "main";
pub const PROFILE_ENV: &str = "TWIST_PROFILE";

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,

    #[structopt(long, short)]
    debug: bool,
}

#[derive(Debug, StructOpt)]
enum Command {
    Add(add::Opt),
    Apply(apply::Opt),
    Init(init::Opt),
    Pull(pull::Opt),
    Push(push::Opt),
    Remove(remove::Opt),
    Update(update::Opt),
}

fn main() {
    let opt = Opt::from_args();

    let mut log_level = LevelFilter::Warn;

    if opt.debug {
        log_level = LevelFilter::Debug;
    }

    TermLogger::init(
        log_level,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .unwrap();

    match opt.command {
        Command::Add(opt) => add::run(opt),
        Command::Apply(opt) => apply::run(opt),
        Command::Init(opt) => init::run(opt),
        Command::Pull(opt) => pull::run(opt),
        Command::Push(opt) => push::run(opt),
        Command::Remove(opt) => remove::run(opt),
        Command::Update(opt) => update::run(opt),
    };
}
