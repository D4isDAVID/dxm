//! Contains the command to prepare an installed third-party resource for patching.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

/// The command structure.
pub fn cli() -> Command {
    Command::new("prepare")
        .about("Prepare an installed third-party resource for patching")
        .arg(
            Arg::new("name")
                .help("The name of the resource to patch")
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

    let (manifest_path, manifest) = crate::util::manifest::find(manifest_path)?;

    if let Some(resource) = manifest.resources.get(name) {
        dxm_resources::patch::prepare(
            resource.patch(&manifest_path),
            resource
                .category(manifest.server.resources(&manifest_path))
                .join(name),
        )?;

        log::info!("you can now edit the resource and run dxm patch commit when you're done");
    } else {
        log::error!("resource {name} is not installed");
    }

    Ok(())
}
