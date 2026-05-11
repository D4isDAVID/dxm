//! Contains the command to remove the third-party FXServer monitor.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::lockfile::Lockfile;

/// The command structure.
pub fn cli() -> Command {
    Command::new("remove")
        .about("Uninstall the third-party FXServer monitor")
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

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    if let Some(monitor) = manifest.artifact.monitor() {
        let platform = ArtifactsPlatform::default();

        crate::util::artifacts::remove_monitor(
            &manifest_path,
            &manifest.artifact,
            &platform,
            monitor,
            &mut lockfile,
        )?;

        manifest.artifact.remove_monitor();
        lockfile.remove_monitor_url();

        manifest.write_artifact(&manifest_path)?;
        lockfile.write(manifest_path)?;

        log::info!("successfully uninstalled resource");
    } else {
        log::error!("there is no third-party monitor is not installed");
    }

    Ok(())
}
