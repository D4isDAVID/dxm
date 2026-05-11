//! Contains commands for managing the third-party FXServer monitor installation.

use std::error::Error;

use clap::{ArgMatches, Command};

pub mod install;
pub mod remove;

/// The command structure.
pub fn cli() -> Command {
    Command::new("monitor")
        .about("Manage the third-party FXServer monitor")
        .subcommand(install::cli())
        .subcommand(remove::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match args.subcommand() {
        Some(("install", m)) => install::execute(m)?,
        Some(("remove", m)) => remove::execute(m)?,
        _ => unreachable!(),
    }

    Ok(())
}
