use clap::{ArgMatches, Command};

use crate::context::CliContext;

mod setup;
mod uninstall;

pub fn cli() -> Command {
    Command::new("self")
        .about("Manage the dxm installation")
        .subcommand(setup::cli())
        .subcommand(uninstall::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

pub fn execute(context: &CliContext, args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("setup", m)) => setup::execute(context, m)?,
        Some(("uninstall", m)) => uninstall::execute(context, m)?,
        _ => unreachable!(),
    }

    Ok(())
}
