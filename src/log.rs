//! Contains code for initializing the logger.

use std::fmt::Arguments;

use fern::FormatCallback;
use log::{Level, LevelFilter, Record};

/// Initializes the logger.
pub fn init() -> Result<(), log::SetLoggerError> {
    let formatter = |out: FormatCallback, message: &Arguments, record: &Record| {
        out.finish(format_args!(
            "{}: {}",
            get_level_format(record.level()),
            message
        ));
    };

    let stdout = fern::Dispatch::new()
        .filter(|data| data.level() >= log::LevelFilter::Info)
        .format(formatter)
        .chain(std::io::stdout());

    let stderr = fern::Dispatch::new()
        .level(LevelFilter::Warn)
        .format(formatter)
        .chain(std::io::stderr());

    fern::Dispatch::new().chain(stdout).chain(stderr).apply()?;

    Ok(())
}

fn get_level_format(level: Level) -> String {
    let s = match level {
        Level::Error => "error",
        Level::Warn => "warn",
        Level::Info => "info",
        Level::Debug => "debug",
        Level::Trace => "trace",
    };

    s.to_owned()
}
