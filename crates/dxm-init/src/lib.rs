//! A crate for initializing FXServer data directories.

use std::{error::Error, path::Path};

use dxm_manifest::Manifest;
use vcs::VcsOption;

pub mod vcs;

pub const SERVER_CFG_NAME: &str = "server.cfg";

/// Initialize server data files in the given directory path with the given vcs.
pub fn server<P>(path: P, vcs: &VcsOption) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    fs_err::create_dir_all(path)?;

    let manifest = Manifest::default();
    let data_path = manifest.server.data(path);

    manifest.write(path)?;

    fs_err::create_dir_all(manifest.server.resources(path))?;
    fs_err::write(data_path.join(SERVER_CFG_NAME), "")?;

    vcs.init(path, &manifest)?;

    Ok(())
}
