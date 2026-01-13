//! A crate for installing third-party resources for FXServer.

use std::{error::Error, io::Write, path::Path};

use fs_extra::dir::{CopyOptions, move_dir};
use reqwest::blocking::Client;
use tempfile::{NamedTempFile, TempDir};
use zip::{ZipArchive, read::root_dir_common_filter};

mod github;

const ROOT_GITIGNORE: &str = "\
*
";

pub fn resolve_download_url<S>(client: &Client, url: S) -> Result<String, Box<dyn Error>>
where
    S: Into<String>,
{
    let url = url.into();

    if let Some(github_url) = github::resolve_download_url(client, &url)? {
        Ok(github_url)
    } else {
        Ok(url)
    }
}

/// Downloads and installs the given archive URL to the given directory path.
/// Returns the archive download URL.
pub fn install<S, P, N>(
    client: &Client,
    download_url: S,
    path: P,
    nested_path: N,
) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
    P: AsRef<Path>,
    N: AsRef<Path>,
{
    let download_url = download_url.as_ref();
    let path = path.as_ref();
    let nested_path = nested_path.as_ref();

    fs_err::create_dir_all(path)?;

    let mut file = NamedTempFile::with_suffix("dxm-resource-archive")?;
    let bytes = client
        .get(download_url)
        .send()?
        .error_for_status()?
        .bytes()?;
    file.write_all(&bytes)?;

    log::debug!("extracting resource into {}", nested_path.display());
    let dir = TempDir::with_suffix("dxm-resource")?;
    ZipArchive::new(file.reopen()?)?.extract_unwrapped_root_dir(&dir, root_dir_common_filter)?;

    let copy_options = CopyOptions::new().content_only(true);
    let mut original_dir: Option<TempDir> = None;

    if is_dir_with_files(path)? {
        let tempdir = TempDir::with_suffix("dxm-resource-original")?;
        move_dir(path, &tempdir, &copy_options)?;
        original_dir = Some(tempdir);
    }

    let result = move_dir(dir.path().join(nested_path), path, &copy_options);
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
