//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
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
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    if args.get_flag("all") {
        crate::util::artifacts::update(
            &client,
            &platform,
            &manifest_path,
            &manifest,
            &mut lockfile,
        )?;
        crate::util::resources::update(&client, &manifest_path, &manifest, &mut lockfile)?;

        lockfile.write(manifest_path)?;

        log::info!("successfully updated resources");
    } else if let Some(resource_name) = args.get_one::<String>("resource") {
        if manifest.resources.contains_key(resource_name) {
            crate::util::resources::update_single(
                &client,
                &manifest_path,
                &manifest,
                &mut lockfile,
                resource_name,
            )?;

            lockfile.write(manifest_path)?;

            log::info!("successfully updated resource");
        } else {
            log::error!("no such resource {}", resource_name);
        }
    } else {
        log::error!("specify either --resource or --all to update");
    }

    Ok(())
}
