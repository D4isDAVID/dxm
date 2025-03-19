//! Contains the command to uninstall dxm.

use clap::{Arg, ArgAction, ArgMatches, Command};
use dxm_home::Home;

/// The command structure.
pub fn cli() -> Command {
    Command::new("uninstall")
        .about("Uninstall dxm")
        .arg(
            Arg::new("env-path")
                .help("Don't modify the environment PATH")
                .long("no-env-path")
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

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> std::io::Result<()> {
    if !args.get_flag("yes") {
        log::info!(
            "are you sure you want to uninstall dxm? \
            run this command with --yes to confirm."
        );
        return Ok(());
    }

    let home = Home::default();

    if !home.exists()? {
        log::info!("there is nothing to uninstall");
        return Ok(());
    }

    log::info!("uninstalling dxm");
    home.uninstall()?;

    if args.get_flag("env-path") {
        if home.in_env_path()? {
            log::info!("removing binaries from environment path");
            home.remove_from_env_path()?;

            log::info!("successfully removed binaries from environment path");
        } else {
            log::info!("binaries are already removed from environment path");
        }
    } else {
        log::trace!("skipped removing binaries from environment path");
    }

    log::info!("successfully uninstalled dxm");

    Ok(())
}
