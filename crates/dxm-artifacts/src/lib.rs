//! A crate for installing and FXServer installations.

use std::{error::Error, io::Write, path::Path};

use cfx::ArtifactsPlatform;
use github::get_version_commit_sha;
use reqwest::blocking::Client;
use tempfile::NamedTempFile;

pub mod cfx;
pub mod github;
pub mod jg;

/// Downloads and installs the given installation version to the given directory
/// path. Returns the installation download URL.
pub fn install<S, P>(
    client: &Client,
    platform: &ArtifactsPlatform,
    version: S,
    path: P,
) -> Result<String, Box<dyn Error>>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    fs_err::create_dir_all(path)?;

    let mut file = NamedTempFile::with_suffix(platform.archive_name())?;
    let url = download(client, platform, version, file.as_file_mut())?;

    log::trace!("extracting fxserver archive");
    platform.decompress(file.reopen()?, path)?;

    Ok(url)
}

/// Downloads the given installation version archive, and writes it to the given
/// writer. Returns the installation download URL.
pub fn download<S, W>(
    client: &Client,
    platform: &ArtifactsPlatform,
    version: S,
    mut writer: W,
) -> Result<String, Box<dyn Error>>
where
    S: AsRef<str>,
    W: Write,
{
    let version = version.as_ref();

    let commit_sha = get_version_commit_sha(client, version)?;
    let url = platform.runtime_url(version, commit_sha);

    log::trace!("download fxserver archive");
    let bytes = client.get(&url).send()?.bytes()?;
    writer.write_all(&bytes)?;

    Ok(url)
}
