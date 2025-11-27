//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use dxm_manifest::lockfile::Lockfile;

/// The command structure.
pub fn cli() -> Command {
    Command::new("update")
        .about("Update FXServer and third-party resources")
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(1)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("all")
                .help("Update the FXServer installation and third-party resources")
                .long("all")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("resource")
                .help("The third-party resource to update")
                .long("resource")
                .short('r'),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");

    let (manifest_path, manifest) = crate::util::manifest::find(manifest_path)?;
    let resources_path = &manifest.server.resources(&manifest_path);

    let client = crate::util::reqwest::github_client().build()?;

    if args.get_flag("all") {
        crate::util::artifacts::update(&manifest_path, &manifest)?;

        let mut lockfile = Lockfile::read(&manifest_path)?;

        for (resource_name, resource) in manifest.resources.iter() {
            if let Some(url) = resource.url() {
                log::info!("installing resource {}", resource_name);

                let base_path = resource.category(resources_path);
                let nested_path = resource.nested_path();

                let resource_url =
                    dxm_resources::update(&client, url, base_path, resource_name, nested_path)?;

                lockfile.set_resource_url(resource_name, resource_url);
            } else {
                log::warn!("no download url found for {}", resource_name);
            }
        }

        lockfile.write(manifest_path)?;

        log::info!("successfully updated artifacts and resources");
    } else if let Some(resource_name) = args.get_one::<String>("resource") {
        if let Some(resource) = manifest.resources.get(resource_name) {
            if let Some(url) = resource.url() {
                log::info!("installing resource {}", resource_name);

                let base_path = resource.category(resources_path);
                let nested_path = resource.nested_path();

                let resource_url =
                    dxm_resources::update(&client, url, base_path, resource_name, nested_path)?;

                Lockfile::write_resource_url(manifest_path, resource_name, resource_url)?;
            } else {
                log::warn!("no download url found for {}", resource_name);
            }
        } else {
            log::error!("no such resource {}", resource_name);
        }

        log::info!("successfully updated resource");
    } else {
        log::error!("specify either --resource or --all to update");
    }

    Ok(())
}
