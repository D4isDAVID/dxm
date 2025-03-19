//! Contains the command for initializing FXServer data.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_init::vcs::VcsOption;

/// The command structure.
pub fn cli() -> Command {
    Command::new("new")
        .about("Create a new server")
        .arg(
            Arg::new("path")
                .help("The directory where to create the server")
                .index(1)
                .required(true)
                .value_parser(clap::value_parser!(PathBuf)),
        )
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
    let path = args.get_one::<PathBuf>("path").expect("no path");
    let vcs = args.get_one::<VcsOption>("vcs").expect("no vcs");

    log::info!("creating files");

    dxm_init::server(path, vcs)?;

    let mut manifest = crate::util::manifest::find(&path)?;
    crate::util::artifacts::update(path, &mut manifest)?;

    Ok(())
}
