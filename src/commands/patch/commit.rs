//! Contains the command to patch an installed third-party resource.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};

/// The command structure.
pub fn cli() -> Command {
    Command::new("commit")
        .about("Patch an installed third-party resource")
        .arg(
            Arg::new("name")
                .help("The name of the resource to patch")
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
        .arg(
            Arg::new("patches-dir")
                .long("patches-dir")
                .short('d')
                .help("The path to save the patch to")
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("patches"),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let name = args.get_one::<String>("name").expect("no name");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let patches_dir = args
        .get_one::<PathBuf>("patches-dir")
        .expect("no patches dir");

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;

    if let Some(resource) = manifest.resources.get_mut(name) {
        let resource_path = resource
            .category(manifest.server.resources(&manifest_path))
            .join(name);

        if !dxm_resources::patch::is_prepared(&resource_path) {
            log::info!(
                "resource {name} wasn't prepared for patching; you must run dxm patch prepare first"
            );

            return Ok(());
        }

        let patch_path = manifest_path
            .join(patches_dir)
            .join(format!("{name}.patch"));

        resource.set_patch(&manifest_path, &patch_path)?;
        manifest.write_resources(&manifest_path)?;

        dxm_resources::patch::commit(patch_path, resource_path)?;

        log::info!("successfully patched {name}");
    } else {
        log::error!("resource {name} is not installed");
    }

    Ok(())
}
