//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::lockfile::Lockfile;

/// The command structure.
pub fn cli() -> Command {
    Command::new("install")
        .about("Install FXServer and third-party resources")
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

    let (manifest_path, manifest) = crate::util::manifest::find(manifest_path)?;
    let artifact = &manifest.artifact;

    let mut lockfile = Lockfile::read(&manifest_path)?;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    if let Some(version) = lockfile.artifact_version() {
        log::info!("installing artifact {}", &version);

        dxm_artifacts::install(&client, &platform, &version, artifact.path(&manifest_path))?;
    } else {
        crate::util::artifacts::update(&manifest_path, &manifest)?;

        lockfile = Lockfile::read(&manifest_path)?;
    }

    let resources_path = &manifest.server.resources(&manifest_path);

    for (resource_name, resource) in manifest.resources.iter() {
        log::info!("installing resource {}", resource_name);

        let url = lockfile.get_resource_url(resource_name).or(resource.url());
        let base_path = resource.category(resources_path);
        let nested_path = resource.nested_path();

        if let Some(url) = url {
            let resource_url =
                dxm_resources::install(&client, url, base_path, resource_name, nested_path)?;

            lockfile.set_resource_url(resource_name, resource_url);
        } else {
            log::warn!("no download url found for {}", resource_name);
        }
    }

    lockfile.write(manifest_path)?;

    log::info!("successfully installed artifacts and resources");

    Ok(())
}
