//! Contains commands to patch an installed third-party resource.

use std::error::Error;

use clap::{ArgMatches, Command};

pub mod commit;
pub mod prepare;
pub mod remove;

/// The command structure.
pub fn cli() -> Command {
    Command::new("patch")
        .about("Patch installed third-party resources.")
        .subcommand(commit::cli())
        .subcommand(prepare::cli())
        .subcommand(remove::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    match args.subcommand() {
        Some(("commit", m)) => commit::execute(m)?,
        Some(("prepare", m)) => prepare::execute(m)?,
        Some(("remove", m)) => remove::execute(m)?,
        _ => unreachable!(),
    }

    Ok(())
}
