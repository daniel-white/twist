use std::env;
use std::error::Error;
use std::ffi::OsStr;
use std::fs::File;
use std::path::Path;

use clap_complete::{generate, Shell};

use twist_cli::cli;

fn main() -> Result<(), Box<dyn Error>> {
    let outdir = match env::var_os("OUT_DIR") {
        Some(outdir) => outdir,
        None => return Ok(()),
    };

    for (shell, name, file_name) in [
        (Shell::Bash, "bash", OsStr::new("bash_completions")),
        (Shell::Fish, "fish", OsStr::new("fish_completions")),
        // (Shell::Zsh, "zsh", OsStr::new("zsh_completions")),
    ] {
        let path = Path::new(&outdir).join(file_name);
        let mut file = File::create(&path)?;
        let mut cli = cli();
        generate(shell, &mut cli, "twist", &mut file);

        println!(
            "cargo:info=completion file for {} is generated into {:?}",
            name, path
        );
    }

    // // // for shell in &shells {
    // // //     shell.generate(&cmd, buf)
    // // // }

    // // let path = generate_to(Zsh, &mut cmd, "twist", &outdir)?;
    // // let path = generate_to(Bash, &mut cmd, "twist", &outdir)?;
    // // let path = generate_to(Fish, &mut cmd, "twist", &outdir)?;

    //
    Ok(())
}
