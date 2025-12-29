//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_manifest::lockfile::Lockfile;

/// The command structure.
pub fn cli() -> Command {
    Command::new("remove")
        .about("Uninstall FXServer resources")
        .arg(
            Arg::new("name")
                .help("The name of the resource to uninstall")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(2)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let name = args.get_one::<String>("name").expect("no name");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    if manifest.resources.contains_key(name) {
        crate::util::resources::uninstall_single(&manifest_path, &manifest, &mut lockfile, name)?;

        manifest.resources.remove(name);
        manifest.write_resources(&manifest_path)?;
        lockfile.write(manifest_path)?;

        log::info!("successfully uninstalled resource");
    } else {
        log::error!("resource {} is not installed", name);
    }

    Ok(())
}
