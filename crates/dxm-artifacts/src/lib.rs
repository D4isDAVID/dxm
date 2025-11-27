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
/// path.
pub fn install<S, P>(
    client: &Client,
    platform: &ArtifactsPlatform,
    version: S,
    path: P,
) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let path = path.as_ref();

    fs_err::create_dir_all(path)?;

    let mut file = NamedTempFile::with_suffix(platform.archive_name())?;
    download(client, platform, version, file.as_file_mut())?;

    log::trace!("extracting fxserver archive");
    platform.decompress(file.reopen()?, path)?;

    Ok(())
}

/// Downloads the given installation version archive, and writes it to the given
/// writer.
pub fn download<S, W>(
    client: &Client,
    platform: &ArtifactsPlatform,
    version: S,
    mut writer: W,
) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
    W: Write,
{
    let version = version.as_ref();

    let commit_sha = get_version_commit_sha(client, version)?;
    let url = platform.runtime_url(version, commit_sha);

    log::trace!("download fxserver archive");
    let bytes = client.get(url).send()?.error_for_status()?.bytes()?;
    writer.write_all(&bytes)?;

    Ok(())
}
