use anyhow::Result;
use simplelog::{ColorChoice, ConfigBuilder, LevelFilter, TermLogger, TerminalMode};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggingError {
    #[error("failed to initialize logging")]
    InitFailed,
}

pub fn init(verbose: bool) -> Result<()> {
    TermLogger::init(
        verbose
            .then_some(LevelFilter::Debug)
            .unwrap_or(LevelFilter::Info),
        ConfigBuilder::new()
            .set_time_level(LevelFilter::Off)
            .build(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .or(Err(LoggingError::InitFailed.into()))
}
