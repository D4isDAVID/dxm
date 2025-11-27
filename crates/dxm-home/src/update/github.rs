//! GitHub-specific code for downloading updates.

use std::{error::Error, io::Write};

use reqwest::blocking::Client;
use serde::Deserialize;

use super::platform::UpdatePlatform;

const GITHUB_LATEST_RELEASE_API_URL: &str =
    "https://api.github.com/repos/D4isDAVID/dxm/releases/latest";

/// Represents a GitHub release.
#[derive(Deserialize)]
pub struct Release {
    tag_name: String,
    assets: Vec<ReleaseAsset>,
}

/// Represents a GitHub release asset.
#[derive(Deserialize)]
pub struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

impl Release {
    /// Returns the update archive URL of the release for the given platform.
    pub fn archive_url(&self, platform: &UpdatePlatform) -> Option<&str> {
        let archive_name = platform.archive_name(&self.tag_name);

        log::debug!("finding update archive url: {archive_name}");
        for asset in &self.assets {
            if asset.name == archive_name {
                return Some(&asset.browser_download_url);
            }
        }

        None
    }

    /// Returns the tag name of the release.
    pub fn tag_name(&self) -> &str {
        &self.tag_name
    }
}

/// Fetches and returns the latest release.
pub fn latest_release(client: &Client) -> reqwest::Result<Release> {
    log::trace!("getting latest release");
    client
        .get(GITHUB_LATEST_RELEASE_API_URL)
        .send()?
        .error_for_status()?
        .json::<Release>()
}

/// Downloads the archive of the given release, and writes it to the given
/// writer.
pub fn download_archive<W>(
    client: &Client,
    release: &Release,
    platform: &UpdatePlatform,
    mut writer: W,
) -> Result<(), Box<dyn Error>>
where
    W: Write,
{
    let url = release
        .archive_url(platform)
        .ok_or("couldn't find archive url")?;

    log::trace!("downloading update archive");
    let bytes = client.get(url).send()?.error_for_status()?.bytes()?;
    writer.write_all(&bytes)?;

    Ok(())
}
