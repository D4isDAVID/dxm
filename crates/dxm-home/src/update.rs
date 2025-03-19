//! Contains code for updating dxm.

use std::{error::Error, path::Path};

use github::Release;
pub use platform::UpdatePlatform;
use reqwest::blocking::Client;
use tempfile::{NamedTempFile, TempDir};

pub mod github;
mod platform;

/// Downloads an update to a new temporary directory, and returns the temporary
/// directory.
pub fn download_temp_dir(
    client: &Client,
    release: &Release,
    platform: &UpdatePlatform,
) -> Result<TempDir, Box<dyn Error>> {
    log::trace!("creating temporary update directory");
    let dir = TempDir::with_prefix("dxm")?;

    download_dir(client, release, platform, dir.path())?;

    Ok(dir)
}

/// Downloads an update to the given directory path.
pub fn download_dir<P>(
    client: &Client,
    release: &Release,
    platform: &UpdatePlatform,
    path: P,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    log::trace!("creating temporary file for update archive");
    let mut file = NamedTempFile::with_prefix("dxm")?;

    github::download_archive(client, release, platform, file.as_file_mut())?;

    log::trace!("extracting update archive");
    platform.decompress(file.reopen()?, path)?;

    Ok(())
}
