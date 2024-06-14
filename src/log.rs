use log::{Level, LevelFilter};

pub fn init(filter: LevelFilter) -> anyhow::Result<()> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}: {message}",
                get_level_name(record.level()),
            ));
        })
        .level(filter)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

fn get_level_name(level: Level) -> String {
    let s = match level {
        Level::Error => "error",
        Level::Warn => "warning",
        Level::Info => "info",
        Level::Debug => "debug",
        Level::Trace => "trace",
    };

    s.to_owned()
}
