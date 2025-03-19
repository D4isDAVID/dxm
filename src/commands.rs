//! Contains code for the commands.

use std::error::Error;

use clap::{Arg, ArgAction, ArgMatches, Command};
use log::LevelFilter;

pub mod artifacts;
pub mod run;
pub mod self_cmd;

/// Options passed to the top-level `execute` function.
pub struct ExecuteOptions {
    /// The log level to use by default.
    ///
    /// Default: `Info`
    pub default_log_level: LevelFilter,
    /// The log level to use when the verbose flag is on.
    ///
    /// Default: `Trace`
    pub verbose_log_level: LevelFilter,
    /// The log level to use when the quiet flag is on.
    ///
    /// Default: `Off`
    pub quiet_log_level: LevelFilter,
}

impl Default for ExecuteOptions {
    fn default() -> Self {
        Self {
            default_log_level: LevelFilter::Info,
            verbose_log_level: LevelFilter::Trace,
            quiet_log_level: LevelFilter::Off,
        }
    }
}

/// The command structure.
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
        .subcommand(artifacts::cli())
        .subcommand(run::cli())
        .subcommand(self_cmd::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches, options: &ExecuteOptions) -> Result<(), Box<dyn Error>> {
    log::set_max_level(determine_log_level(args, options));

    match args.subcommand() {
        Some(("artifacts", m)) => artifacts::execute(m)?,
        Some(("run", m)) => run::execute(m)?,
        Some(("self", m)) => self_cmd::execute(m)?,
        _ => unreachable!(),
    }

    Ok(())
}

fn determine_log_level(args: &ArgMatches, options: &ExecuteOptions) -> LevelFilter {
    if args.get_flag("quiet") {
        options.quiet_log_level
    } else if args.get_flag("verbose") {
        options.verbose_log_level
    } else {
        options.default_log_level
    }
}
