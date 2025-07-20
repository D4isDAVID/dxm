//! Contains the command to update FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

/// The command structure.
pub fn cli() -> Command {
    Command::new("update")
        .about("Update FXServer artifacts")
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(1)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");

    let mut manifest = crate::util::manifest::find(manifest_path)?;

    crate::util::artifacts::update(manifest_path, &mut manifest)?;

    Ok(())
}
