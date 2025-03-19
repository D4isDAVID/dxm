//! Contains the command to start a server.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;

/// The command structure.
pub fn cli() -> Command {
    Command::new("run")
        .about("Start FXServer")
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(1)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("server-args")
                .help("Extra args passed to FXServer")
                .index(2)
                .last(true),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let server_args = args
        .get_many::<String>("server-args")
        .map_or_else(Vec::new, Iterator::collect);

    let mut manifest = crate::util::manifest::find(manifest_path)?;

    let artifact = &manifest.artifact;
    let platform = ArtifactsPlatform::default();
    let exe = artifact.exe(manifest_path, platform);

    let server = &mut manifest.server;
    let data = server.ensure_data(manifest_path)?;

    log::debug!("running server with {}", exe.display());
    log::debug!("using data path {}", data.display());

    let mut command = std::process::Command::new(exe);
    command
        .current_dir(&data)
        .args(server_args)
        .spawn()?
        .wait()?;

    Ok(())
}
