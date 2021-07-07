use crate::{DEFAULT_PROFILE, PROFILE_ENV};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opt {
    #[structopt(long, short, env = PROFILE_ENV, default_value = DEFAULT_PROFILE)]
    profile: String,

    #[structopt(long, short = "m")]
    message: Option<String>,
}

pub fn run(opt: Opt) {
    println!("{:?}", opt);
}
