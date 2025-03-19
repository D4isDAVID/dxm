//! A crate for initializing FXServer data directories.

use std::{error::Error, path::Path};

use dxm_manifest::Manifest;
use vcs::VcsOption;

pub mod vcs;

/// Initialize server data files in the given directory path with the given vcs.
pub fn server<P>(path: P, vcs: &VcsOption) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    fs_err::create_dir_all(path)?;

    Manifest::default().write(path)?;

    fs_err::create_dir_all(path.join("data").join("resources"))?;
    fs_err::write(path.join("data").join("server.cfg"), "")?;

    vcs.init(path)?;

    Ok(())
}
