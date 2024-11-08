use clap::{ArgMatches, Command};

use crate::context::CliContext;

mod list;

pub fn cli() -> Command {
    Command::new("artifact")
        .about("Manage the server artifacts")
        .subcommand(list::cli())
        .arg_required_else_help(true)
        .subcommand_required(true)
}

pub fn execute(context: &mut CliContext, args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("list", _)) => list::execute()?,
        _ => unreachable!(),
    }

    Ok(())
}
