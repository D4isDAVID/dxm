//! Contains the command to revert a patch for an installed third-party resource.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

/// The command structure.
pub fn cli() -> Command {
    Command::new("remove")
        .about("Remove a patch for an installed third-party resource")
        .arg(
            Arg::new("name")
                .help("The name of the patched resource")
                .index(1),
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

    if let Some(resource) = manifest.resources.get_mut(name) {
        if let Some(patch) = resource.patch(&manifest_path) {
            let resource_path = resource
                .category(manifest.server.resources(&manifest_path))
                .join(name);

            resource.remove_patch();
            manifest.write_resources(&manifest_path)?;

            dxm_resources::patch::remove(patch, resource_path)?;

            log::info!("successfully removed patch");
        } else {
            log::error!("resource {name} is not patched");
        }
    } else {
        log::error!("resource {name} is not installed");
    }

    Ok(())
}
