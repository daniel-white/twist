use anyhow::Result;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("failed to initialize logging")]
    InitFailed,
}

pub fn init(verbose: bool) -> Result<()> {
    let log_level = if verbose {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    TermLogger::init(
        log_level,
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .map_err(|_| LoggingError::InitFailed.into())
}
