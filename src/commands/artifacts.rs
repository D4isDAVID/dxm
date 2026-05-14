//! Contains commands for managing FXServer installations.

use std::error::Error;

use clap::{ArgMatches, Command};

pub mod install;
pub mod list;

/// The command structure.
pub fn cli() -> Command {
    Command::new("artifacts")
        .about("Manage FXServer artifacts")
        .subcommand(install::cli())
        .subcommand(list::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match args.subcommand() {
        Some(("install", m)) => install::execute(m)?,
        Some(("list", _)) => list::execute()?,
        _ => unreachable!(),
    }

    Ok(())
}
