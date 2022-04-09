use std::env;

use clap::Error;
use clap_complete::{generate_to, shells::Zsh};

use twist_cli::into_command;

fn main() -> Result<(), Error> {
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = into_command();
    let path = generate_to(Zsh, &mut cmd, "twist", outdir)?;

    println!("cargo:warning=completion file is generated: {:?}", path);

    Ok(())
}
