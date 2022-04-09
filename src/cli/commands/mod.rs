use anyhow::Result;
use log::error;
use structopt::StructOpt;

mod add;
mod apply;
mod init;
mod pull;
mod push;
mod remove;
mod update;

#[derive(Debug, StructOpt)]
pub enum CommandVerb {
    #[structopt(about = "Adds the given files and directories to the dotfiles repository")]
    Add(add::Add),
    #[structopt(about = "Applies the dotfiles to the current system")]
    Apply(apply::Apply),
    #[structopt(about = "Initializes the dotfiles repository")]
    Init(init::Init),
    #[structopt(about = "Pulls the dotfiles from the remote repository")]
    Pull(pull::Pull),
    #[structopt(about = "Pushes the dotfiles to the remote repository")]
    Push(push::Push),
    #[structopt(
        about = "Removes the given files and directories from the dotfiles repository",
        alias = "rm"
    )]
    Remove(remove::Remove),
    #[structopt(about = "Updates the dotfiles repository from the current system")]
    Update(update::Update),
}

trait Command {
    fn run(&self) -> Result<()>;
}

impl CommandVerb {
    pub fn run(&self) -> Result<()> {
        match self {
            CommandVerb::Add(command) => command.run(),
            CommandVerb::Apply(command) => command.run(),
            CommandVerb::Init(command) => command.run(),
            CommandVerb::Pull(command) => command.run(),
            CommandVerb::Push(command) => command.run(),
            CommandVerb::Remove(command) => command.run(),
            CommandVerb::Update(command) => command.run(),
        }
        .map_err(|err| {
            error!("{}", err);
            err
        })
    }
}
