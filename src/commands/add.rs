//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgMatches, Command};
use dxm_manifest::{lockfile::Lockfile, resource::Resource};

/// The command structure.
pub fn cli() -> Command {
    Command::new("add")
        .about("Install FXServer resources")
        .arg(
            Arg::new("name")
                .help("The name of the resource to install")
                .index(1)
                .required(true),
        )
        .arg(
            Arg::new("url")
                .help("The download URL of the resource to install")
                .index(2)
                .required(true),
        )
        .arg(
            Arg::new("manifest-path")
                .help("The path to a directory with a dxm manifest")
                .index(3)
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
        .arg(
            Arg::new("category")
                .help("The category to install the resource to")
                .long("category")
                .short('c'),
        )
        .arg(
            Arg::new("nested-path")
                .help("The path to the resource inside the download archive")
                .long("nested-path")
                .short('n')
                .value_parser(clap::value_parser!(PathBuf))
                .default_value("."),
        )
}

/// The code ran when using the command.
pub fn execute(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let name = args.get_one::<String>("name").expect("no name");
    let url = args.get_one::<String>("url").expect("no url");
    let manifest_path = args
        .get_one::<PathBuf>("manifest-path")
        .expect("no manifest path");
    let category = args
        .get_one::<String>("category")
        .filter(|c| !c.contains('.'))
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from("."));
    let nested_path = args
        .get_one::<PathBuf>("nested-path")
        .expect("no nested path");

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let resources = &mut manifest.resources;
    let server_resources = &manifest.server.resources(&manifest_path);
    let base_path = server_resources.join(&category);

    let client = crate::util::reqwest::client().build()?;
    resources.insert(name.clone(), Resource::new(url, category, nested_path));

    log::info!("installing resource {}", &name);

    let resource_url = dxm_resources::install(&client, url, base_path, name, nested_path)?;
    manifest.write_resources(&manifest_path)?;
    Lockfile::write_resource_url(manifest_path, name, resource_url)?;

    log::info!("successfully installed resource");

    Ok(())
}
