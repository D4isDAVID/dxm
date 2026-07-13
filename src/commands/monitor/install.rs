//! Contains the command to install a third-party FXServer monitor.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::{lockfile::Lockfile, resource::Resource};

/// The command structure.
pub fn cli() -> Command {
    Command::new("install")
        .about("Install a third-party FXServer monitor")
        .arg(
            Arg::new("url")
                .help("The download URL of the monitor to install")
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
            Arg::new("nested-path")
                .help("The path to the monitor inside the download archive")
                .long("nested-path")
                .short('n'),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let url = args.get_one::<String>("url").expect("no url");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let nested_path = args
        .get_one::<String>("nested-path")
        .filter(|n| !n.contains('.'))
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(""));

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    if manifest.artifact.monitor().is_some() {
        log::error!("a third-party monitor is already installed");

        return Ok(());
    }

    let resource = Resource::new(url, PathBuf::from(""), &nested_path);

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    crate::util::artifacts::install_monitor(
        &client,
        &manifest_path,
        &manifest,
        &platform,
        &resource,
        &mut lockfile,
    )?;

    manifest.artifact.set_monitor(resource);

    manifest.write_artifact(&manifest_path)?;
    lockfile.write(&manifest_path)?;

    log::info!("successfully installed resource");

    Ok(())
}
