use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::context::CliContext;

mod artifact;
mod run;
mod self_cmd;

pub fn cli() -> Command {
    Command::new(clap::crate_name!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .arg(
            Arg::new("verbose")
                .help("Print trace and debug logs")
                .long("verbose")
                .short('v')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .arg(
            Arg::new("quiet")
                .help("Don't print any logs")
                .long("quiet")
                .short('q')
                .action(ArgAction::SetTrue)
                .global(true),
        )
        .subcommand(artifact::cli())
        .subcommand(run::cli())
        .subcommand(self_cmd::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

pub fn execute(args: &ArgMatches) -> anyhow::Result<()> {
    crate::log::init(determine_log_level(args))?;

    log::trace!("initializing context");
    let mut context = CliContext::new_default()?;

    match args.subcommand() {
        Some(("artifact", m)) => artifact::execute(&mut context, m)?,
        Some(("run", m)) => run::execute(&mut context, m)?,
        Some(("self", m)) => self_cmd::execute(&context, m)?,
        _ => unreachable!(),
    }

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
