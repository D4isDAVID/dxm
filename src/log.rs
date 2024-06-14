use log::{Level, LevelFilter};

pub fn init(filter: LevelFilter) -> anyhow::Result<()> {
    fern::Dispatch::new()
        .format(move |out, message, record| {
            let mut prefix = "".to_owned();

            if record.level() <= Level::Warn {
                prefix = get_level_name(record.level());
                prefix.push_str(": ");
            }

            out.finish(format_args!("{prefix}{message}"));
        })
        .level(filter)
        .chain(std::io::stdout())
        .apply()?;

    Ok(())
}

fn get_level_name(level: log::Level) -> String {
    let s = match level {
        Level::Error => "error",
        Level::Warn => "warning",
        Level::Info => "info",
        Level::Debug => "debug",
        Level::Trace => "trace",
    };

    s.to_owned()
}
