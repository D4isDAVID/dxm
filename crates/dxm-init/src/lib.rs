//! A crate for initializing FXServer data directories.

use std::{error::Error, path::Path};

use dxm_manifest::{Manifest, resource::Resource};
use vcs::VcsOption;

pub mod vcs;

pub const README_NAME: &str = "README.md";
pub const ENV_CFG_NAME: &str = "env.cfg";
pub const PERMISSIONS_CFG_NAME: &str = "permissions.cfg";
pub const RESOURCES_CFG_NAME: &str = "resources.cfg";
pub const SECRETS_CFG_NAME: &str = "secrets.cfg";
pub const SERVER_CFG_NAME: &str = "server.cfg";
pub const TXDATA_DIR: &str = "txData";
pub const TXDATA_DEFAULT_PROFILE: &str = "default";
pub const TXDATA_CONFIG_NAME: &str = "config.json";

const README: &str = include_str!("../templates/README.md");
const ENV_CFG: &str = include_str!("../templates/env.cfg");
const PERMISSIONS_CFG: &str = include_str!("../templates/permissions.cfg");
const RESOURCES_CFG: &str = include_str!("../templates/resources.cfg");
const SECRETS_CFG: &str = include_str!("../templates/secrets.cfg");
const SERVER_CFG: &str = include_str!("../templates/server.cfg");
const TXDATA_DEFAULT_CONFIG: &str = include_str!("../templates/txData-config.json");

/// Initialize server data files in the given directory path with the given vcs.
pub fn server<P>(path: P, vcs: &VcsOption) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    fs_err::create_dir_all(path)?;

    let mut manifest = Manifest::default();
    let data_path = manifest.server.data(path);

    manifest.resources.insert(
        "[cfx-default]".to_owned(),
        Resource::new(
            "https://github.com/citizenfx/cfx-server-data",
            "",
            "resources",
        ),
    );

    manifest.write(path)?;
    fs_err::write(path.join(README_NAME), README)?;

    fs_err::create_dir_all(manifest.server.resources(path))?;
    fs_err::write(data_path.join(ENV_CFG_NAME), ENV_CFG)?;
    fs_err::write(data_path.join(PERMISSIONS_CFG_NAME), PERMISSIONS_CFG)?;
    fs_err::write(data_path.join(RESOURCES_CFG_NAME), RESOURCES_CFG)?;
    fs_err::write(data_path.join(SECRETS_CFG_NAME), SECRETS_CFG)?;
    fs_err::write(data_path.join(SERVER_CFG_NAME), SERVER_CFG)?;

    let txdata_default_path = path.join(TXDATA_DIR).join(TXDATA_DEFAULT_PROFILE);

    fs_err::create_dir_all(&txdata_default_path)?;
    fs_err::write(
        txdata_default_path.join(TXDATA_CONFIG_NAME),
        TXDATA_DEFAULT_CONFIG,
    )?;

    vcs.init(path, &manifest)?;

    Ok(())
}
