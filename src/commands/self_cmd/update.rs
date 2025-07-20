//! Contains the command to update dxm.

use std::error::Error;

use clap::{ArgMatches, Command};
use dxm_home::{
    Home,
    update::{UpdatePlatform, github::latest_release},
};

/// The command structure.
pub fn cli() -> Command {
    Command::new("update").about("Update dxm")
}

/// The code ran when using the command.
pub fn execute(_args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let client = crate::util::reqwest::github_client().build()?;
    let home = Home::default();
    let platform = UpdatePlatform::default();

    log::info!("getting latest release");
    let release = latest_release(&client)?;
    let tag_name = release.tag_name();
    let current = concat!("v", clap::crate_version!());

    if tag_name == current {
        log::info!("you already on the latest version");
        return Ok(());
    }

    log::info!("updating dxm to {tag_name}");
    home.update(&client, &release, &platform)?;

    log::info!("successfully updated dxm");

    Ok(())
}
