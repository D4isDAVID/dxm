//! Contains the command for initializing FXServer data in the current directory.

use std::error::Error;

use clap::{Arg, ArgMatches, Command};
use dxm_init::vcs::VcsOption;

/// The command structure.
pub fn cli() -> Command {
    Command::new("init")
        .about("Create a new server in the current directory")
        .arg(
            Arg::new("vcs")
                .help("The version control system to use (git, none)")
                .long("vcs")
                .default_value("git")
                .value_parser(clap::value_parser!(VcsOption)),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let vcs = args.get_one::<VcsOption>("vcs").expect("no vcs");

    let path = std::env::current_dir()?;

    log::info!("creating files");

    dxm_init::server(&path, vcs)?;

    let (path, mut manifest) = crate::util::manifest::find(path)?;
    crate::util::artifacts::update(path, &mut manifest)?;

    Ok(())
}
