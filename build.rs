use std::env;
use std::error::Error;

use clap_complete::{generate_to, shells::Zsh};

use twist_cli::into_command;

fn main() -> Result<(), Box<dyn Error>> {
    shadow_rs::new().map_err(|err| err.to_string())?;

    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = into_command();
    let path = generate_to(Zsh, &mut cmd, "twist", outdir)?;

    println!("cargo:warning=completion file is generated: {:?}", path);

    Ok(())
}
