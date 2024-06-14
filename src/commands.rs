use clap::{Arg, ArgAction, ArgMatches, Command};

pub fn cli() -> Command {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::new("verbose")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
                .global(true),
        )
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    #[allow(clippy::needless_borrow)]
    crate::log::init(determine_log_level(&args))?;

    Ok(())
}

fn determine_log_level(args: &ArgMatches) -> log::LevelFilter {
    if args.get_flag("quiet") {
        log::LevelFilter::Off
    } else if args.get_flag("verbose") {
        log::LevelFilter::Trace
    } else {
        log::LevelFilter::Info
    }
}
