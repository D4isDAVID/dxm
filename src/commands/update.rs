//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::lockfile::Lockfile;

/// The command structure.
pub fn cli() -> Command {
    Command::new("update")
        .about("Update FXServer, third-party resources and monitor")
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(1)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("artifacts")
                .help("Update the FXServer artifacts")
                .long("artifacts")
                .short('a')
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("resources")
                .help("Update the specified third-party resources")
                .long("resources")
                .short('r')
                .num_args(0..),
        )
        .arg(
            Arg::new("monitor")
                .help("Update the third-party FXServer monitor")
                .long("monitor")
                .short('m')
                .action(ArgAction::SetTrue),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let artifacts = args.get_flag("artifacts");
    let resources = args
        .get_many::<String>("resources")
        .map(Iterator::collect::<Vec<_>>);
    let monitor = args.get_flag("monitor");

    let update_all = !artifacts && resources.is_none() && !monitor;

    let (manifest_path, manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    if artifacts || update_all {
        crate::util::artifacts::update(
            &client,
            &platform,
            &manifest_path,
            &manifest,
            &mut lockfile,
        )?;
    }

    if let Some(resources) = resources.or_else(|| if update_all { Some(Vec::new()) } else { None })
    {
        crate::util::resources::update(
            &client,
            &manifest_path,
            &manifest,
            &mut lockfile,
            resources,
        )?;

        log::info!("successfully updated resources");
    }

    if monitor || update_all {
        if let Some(monitor) = manifest.artifact.monitor() {
            crate::util::artifacts::update_monitor(
                &client,
                &manifest_path,
                &manifest.artifact,
                &platform,
                monitor,
                &mut lockfile,
            )?;
        } else if monitor {
            log::info!("no third-party monitor installed");
        }
    }

    lockfile.write(manifest_path)?;

    Ok(())
}
