//! Contains the command to update FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::lockfile::Lockfile;

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

    let (manifest_path, manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    crate::util::artifacts::update(&client, &platform, manifest_path, &manifest, &mut lockfile)?;

    Ok(())
}
