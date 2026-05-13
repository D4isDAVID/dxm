//! Contains the command to start a server.

use std::{collections::HashMap, error::Error, path::PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command};
use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_manifest::profile::{Profile, TXHOST_DATA_PATH};

/// The command structure.
pub fn cli() -> Command {
    Command::new("start")
        .about("Start FXServer")
        .arg(
            Arg::new("profile")
                .help("The path to a directory with a dxm manifest")
                .index(1)
                .default_value("default"),
        )
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(2)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("server-args")
                .help("Extra args passed to FXServer")
                .index(3)
                .last(true)
                .num_args(..),
        )
        .arg(
            Arg::new("env")
                .help("Overwrite an environment variable for FXServer")
                .long("env")
                .short('e')
                .num_args(2)
                .value_names(["key", "value"])
                .action(ArgAction::Append),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let profile = args.get_one::<String>("profile").expect("no profile");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let mut server_args = args
        .get_many::<String>("server-args")
        .map_or_else(Vec::new, Iterator::collect);
    let env_vars: HashMap<&String, &String> = args
        .get_many::<String>("env")
        .map_or_else(Vec::new, Iterator::collect)
        .chunks_exact(2)
        .map(|chunk| (chunk[0], chunk[1]))
        .collect();

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;

    if profile == "default" && !manifest.profiles.contains_key("default") {
        manifest
            .profiles
            .insert("default".to_owned(), Profile::default());
    }

    if let Some(profile) = manifest.profiles.get_mut(profile) {
        let artifact = &manifest.artifact;
        let platform = ArtifactsPlatform::default();
        let exe = artifact.exe(&manifest_path, &platform).canonicalize()?;

        let server = &mut manifest.server;
        let data = server.ensure_data(&manifest_path)?;

        log::debug!("starting server with {}", exe.display());
        log::debug!("using data path {}", data.display());

        if let Some(txdata_path) = env_vars.get(&TXHOST_DATA_PATH.to_owned()) {
            profile.set_txhost_data_path(*txdata_path);
        }

        server_args.append(&mut profile.server_args());
        let mut profile_vars = profile.env_vars();
        profile_vars.extend(env_vars);

        let txdata_path = profile.txhost_data_path(manifest_path)?;
        log::debug!("using txdata path {}", txdata_path.display());

        std::process::Command::new(exe)
            .current_dir(&data)
            .args(server_args)
            .envs(profile_vars)
            .env(TXHOST_DATA_PATH, txdata_path)
            .spawn()?
            .wait()?;
    } else {
        log::error!("profile {profile} does not exist");
    }

    Ok(())
}
