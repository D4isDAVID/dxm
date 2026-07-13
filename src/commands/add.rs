//! Contains the command to install third-party resources for FXServer.

use std::{error::Error, path::PathBuf};

use clap::{Arg, ArgGroup, ArgMatches, Command};
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
                .short('n'),
        )
        .arg(
            Arg::new("git")
                .help("Install the resource from a Git repository")
                .long("git")
                .short('g')
                .num_args(0..=1)
                .value_name("rev")
                .default_missing_value(""),
        )
        .group(ArgGroup::new("source").arg("git"))
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
        .unwrap_or(PathBuf::from(""));
    let nested_path = args
        .get_one::<String>("nested-path")
        .filter(|n| !n.contains('.'))
        .map(PathBuf::from)
        .unwrap_or(PathBuf::from(""));
    let git_rev = args.get_one::<String>("git");

    let (manifest_path, mut manifest) = crate::util::manifest::find(manifest_path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let resources = &mut manifest.resources;

    if resources.contains_key(name) {
        log::error!("resource {} already exists in this server", name);

        return Ok(());
    }

    if let Some(git_rev) = git_rev {
        resources.insert(
            name.to_owned(),
            Resource::new(
                dxm_resources::format_git_url(url, git_rev),
                category,
                &nested_path,
            ),
        );
    } else {
        resources.insert(name.to_owned(), Resource::new(url, category, &nested_path));
    }

    let client = crate::util::reqwest::github_client().build()?;
    let lock_url = crate::util::resources::install_single(
        &client,
        &manifest_path,
        manifest.server.resources(&manifest_path),
        resources,
        &resources[name],
        lockfile.get_resource_url(name),
        name,
    )?;

    if let Some(lock_url) = lock_url {
        lockfile.set_resource_url(name, lock_url);
    }

    manifest.write_resources(&manifest_path)?;
    lockfile.write(&manifest_path)?;

    log::info!("successfully installed resource");

    Ok(())
}
