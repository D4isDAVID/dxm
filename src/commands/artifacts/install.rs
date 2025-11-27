//! Contains the command to install FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
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
    let version_arg = args.get_one::<String>("version");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let path = args.get_one::<String>("path").map(PathBuf::from);

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let artifact = &mut manifest.artifact;

    let mut version = if let Some(version) = version_arg {
        artifact.set_version(version.clone());
        version.clone()
    } else {
        artifact.version().to_owned()
    };

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    if let Some(channel) = artifact.channel() {
        log::info!("getting versions");

        if channel == ArtifactsChannel::LatestJg {
            let artifacts = dxm_artifacts::jg::artifacts(&client)?;
            version = artifacts.version().to_owned();
        } else {
            let versions = dxm_artifacts::cfx::versions(&client, &platform)?;
            version = versions.version(&channel).to_owned();
        }
    }

    let path = path.unwrap_or_else(|| artifact.path(&manifest_path));
    artifact.set_path(&manifest_path, &path)?;

    log::info!("installing artifact {}", &version);

    let artifact_url = dxm_artifacts::install(&client, &platform, &version, path)?;
    manifest.write_artifact(&manifest_path)?;
    Lockfile::write_artifact_url(manifest_path, artifact_url)?;

    log::info!("successfully installed artifact");

    Ok(())
}
