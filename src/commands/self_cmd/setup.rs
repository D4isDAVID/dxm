//! Contains the command to setup dxm.

use clap::{Arg, ArgAction, ArgMatches, Command};
use dxm_home::Home;

/// The command structure.
pub fn cli() -> Command {
    Command::new("setup")
        .about("Set up the dxm installation")
        .arg(
            Arg::new("env-path")
                .help("Don't modify the environment PATH")
                .long("no-env-path")
                .action(ArgAction::SetFalse),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> std::io::Result<()> {
    let home = Home::default();
    let current_exe = std::env::current_exe()?;

    log::info!("setting up dxm");
    home.setup(current_exe)?;

    if args.get_flag("env-path") {
        if home.in_env_path()? {
            log::trace!("binaries are already in environment path");
        } else {
            log::info!("adding binaries to environment path");
            home.add_to_env_path()?;

            log::info!(
                "successfully added binaries to environment path, \
                restart your shell to apply changes"
            );
        }
    } else {
        log::trace!("skipped adding binaries to environment path");
    }

    log::info!("successfully set up dxm");

    Ok(())
}
