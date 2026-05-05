//! A crate for installing third-party resources for FXServer.

use std::{error::Error, path::Path};

use fs_extra::dir::{CopyOptions, move_dir};
use reqwest::blocking::Client;
use tempfile::TempDir;

use crate::download::DownloadSource;

mod download;
mod resolve;

const ROOT_GITIGNORE: &str = "\
*
";

pub fn resolve<'a, S>(
    client: &'a Client,
    download_url: S,
) -> Result<DownloadSource<'a>, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let download_url = download_url.as_ref();

    resolve::download_url(client, download_url)
}

/// Downloads and installs the given archive URL to the given directory path.
/// Returns the archive download URL.
pub fn install<P, N>(source: &DownloadSource, path: P, nested_path: N) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    N: AsRef<Path>,
{
    let path = path.as_ref();
    let nested_path = nested_path.as_ref();

    fs_err::create_dir_all(path)?;

    let copy_options = CopyOptions::new().content_only(true);
    let mut original_dir: Option<TempDir> = None;

    if is_dir_with_files(path)? {
        let tempdir = TempDir::with_suffix("dxm-resource-original")?;
        move_dir(path, &tempdir, &copy_options)?;
        original_dir = Some(tempdir);
    }

    let result = source.download(path, nested_path);
    if result.is_err()
        && let Some(original_dir) = original_dir
    {
        move_dir(original_dir, path, &copy_options)?;
    }

    result?;
    fs_err::write(path.join(".gitignore"), ROOT_GITIGNORE)?;

    Ok(())
}

fn is_dir_with_files<P>(path: P) -> std::io::Result<bool>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if !path.is_dir() {
        return Ok(false);
    };

    for entry in fs_err::read_dir(path)? {
        if entry?.file_type()?.is_file() {
            return Ok(true);
        }
    }

    Ok(false)
}
