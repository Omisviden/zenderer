use anyhow::{Context, Result};
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;

pub const LOG_FILE: &str = "obsidian.log";

pub fn create_logger() -> Result<()> {
    let file = File::create(LOG_FILE).context(format!(
        "Failed to create log file named: {}",
        LOG_FILE.to_string()
    ))?;
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed),
        WriteLogger::new(LevelFilter::max(), Config::default(), file),
    ])?;
    Ok(())
}
