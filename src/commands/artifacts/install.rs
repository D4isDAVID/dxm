//! Contains the command to install FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::lockfile::Lockfile;

/// The command structure.
pub fn cli() -> Command {
    Command::new("install")
        .about("Install FXServer artifacts")
        .arg(
            Arg::new("version")
                .help("The artifacts version to install")
                .index(1),
        )
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(2)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("path")
                .help("The directory to install the artifacts to")
                .long("path")
                .short('p'),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let version = args.get_one::<String>("version");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let path = args.get_one::<String>("path").map(PathBuf::from);

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let artifact = &mut manifest.artifact;

    if let Some(version) = version {
        artifact.set_version(version.clone());
    };

    if let Some(path) = path {
        artifact.set_path("", path)?;
    };

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    crate::util::artifacts::update(&client, &platform, &manifest_path, &manifest, &mut lockfile)?;

    manifest.write_artifact(&manifest_path)?;
    lockfile.write(manifest_path)?;

    Ok(())
}
