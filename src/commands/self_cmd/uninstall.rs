use clap::{Arg, ArgAction, ArgMatches, Command};

use crate::context::CliContext;

pub fn cli() -> Command {
    Command::new("uninstall")
        .about("Uninstall dxm.")
        .arg(
            Arg::new("path")
                .help("Don't modify the environment PATH")
                .long("no-path")
                .action(ArgAction::SetFalse),
        )
        .arg(
            Arg::new("yes")
                .help("Confirm uninstall")
                .long("yes")
                .short('y')
                .action(ArgAction::SetTrue),
        )
}

pub fn execute(context: &CliContext, args: &ArgMatches) -> anyhow::Result<()> {
    if !args.get_flag("yes") {
        log::info!(
            "are you sure you want to uninstall dxm? \
            run this command with --yes to confirm."
        );
        return Ok(());
    }

    if !context.home().path().try_exists()? {
        log::info!("nothing to uninstall");
        return Ok(());
    }

    if args.get_flag("path") {
        log::info!("removing binaries from environment path");
        match context.env().remove() {
            Ok(true) => log::info!(
                "binaries removed from environment path, \
                restart your shell to apply changes"
            ),
            Ok(false) => log::info!("binaries already not in environment path"),
            Err(e) => {
                log::error!("{e}");
                log::info!("failed to remove binaries from environment path");
            }
        }
    } else {
        log::trace!("skipped removing binaries from environment path");
    }

    log::info!("uninstalling dxm");
    context.uninstall()?;

    log::info!("uninstall successful");

    Ok(())
}
