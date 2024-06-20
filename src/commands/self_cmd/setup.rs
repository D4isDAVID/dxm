use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::context::CliContext;

pub fn cli() -> Command {
    Command::new("setup")
        .about("Set up the dxm installation")
        .arg(
            Arg::new("path")
                .help("Don't modify the environment PATH")
                .long("no-path")
                .action(ArgAction::SetFalse),
        )
}

pub fn execute(context: &CliContext, args: &ArgMatches) -> anyhow::Result<()> {
    log::info!("setting up home");
    context.setup_home()?;

    log::info!("setup successful");

    if args.get_flag("path") {
        log::info!("adding binaries to environment path");
        match context.env().add() {
            Ok(true) => log::info!(
                "binaries added to environment path, \
                restart your shell to apply changes"
            ),
            Ok(false) => log::info!("binaries are already in environment path"),
            Err(e) => {
                log::error!("{e}");
                log::info!("failed to add binaries to environment path");
            }
        }
    } else {
        log::trace!("skipped adding binaries to environment path");
    }

    Ok(())
}
