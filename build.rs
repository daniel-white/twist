#[allow(unused_imports)]
use clap::{Command, CommandFactory};

include!("src/cli.rs");

use clap_complete::generate_to;
use clap_complete::shells::Bash;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut command: Command = Cli::command();

    generate_to(Bash, &mut command, "twist", &outdir)?;

    Ok(())
}
