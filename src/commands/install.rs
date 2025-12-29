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
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    crate::util::artifacts::install(&client, &platform, &manifest_path, &manifest, &mut lockfile)?;
    crate::util::resources::install(&client, &manifest_path, &manifest, &mut lockfile)?;

    lockfile.write(manifest_path)?;

    log::info!("successfully installed resources");

    Ok(())
}
