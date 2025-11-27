//! A crate for installing third-party resources for FXServer.

use std::{error::Error, fmt::Display, io::Write, path::Path};

use reqwest::blocking::Client;
use tempfile::{NamedTempFile, TempDir};
use zip::{ZipArchive, read::root_dir_common_filter};

mod github;

const ROOT_GITIGNORE: &str = "\
*
";

#[derive(Debug)]
pub struct InvalidResourceNameError;

impl Display for InvalidResourceNameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid resource name")?;

        Ok(())
    }
}

impl Error for InvalidResourceNameError {}

/// Downloads and installs the given archive URL to the given directory path.
/// Returns the archive download URL.
pub fn install<U, P, S, N>(
    client: &Client,
    url: U,
    base_path: P,
    name: S,
    nested_path: N,
) -> Result<String, Box<dyn Error>>
where
    U: AsRef<str>,
    P: AsRef<Path>,
    S: AsRef<str>,
    N: AsRef<Path>,
{
    let url = url.as_ref();
    let base_path = base_path.as_ref();
    let name = name.as_ref();
    let nested_path = nested_path.as_ref();

    let path = base_path.join(name);

    if path.components().count() > base_path.components().count() + 1 {
        Err(InvalidResourceNameError {})?;
    }

    let url = github::resolve_download_url(client, url)?;

    fs_err::create_dir_all(&path)?;

    let mut file = NamedTempFile::with_suffix(name)?;
    let bytes = client.get(&url).send()?.bytes()?;
    file.write_all(&bytes)?;

    log::debug!("extracting {} into {}", name, nested_path.display());
    let dir = TempDir::with_suffix(name)?;
    ZipArchive::new(file.reopen()?)?.extract_unwrapped_root_dir(&dir, root_dir_common_filter)?;

    fs_err::rename(dir.path().join(nested_path), &path)?;
    fs_err::write(path.join(".gitignore"), ROOT_GITIGNORE)?;

    Ok(url)
}

pub fn update<U, P, S, N>(
    client: &Client,
    url: U,
    base_path: P,
    name: S,
    nested_path: N,
) -> Result<String, Box<dyn Error>>
where
    U: AsRef<str>,
    P: AsRef<Path>,
    S: AsRef<str>,
    N: AsRef<Path>,
{
    let base_path = base_path.as_ref();
    let name = name.as_ref();

    let path = base_path.join(name);

    if path.components().count() > base_path.components().count() + 1 {
        Err(InvalidResourceNameError {})?;
    }

    let dir = TempDir::with_suffix(name)?;
    fs_err::rename(&path, &dir)?;

    match install(client, url, base_path, name, nested_path) {
        Ok(url) => Ok(url),
        Err(err) => {
            fs_err::rename(dir, path)?;

            Err(err)
        }
    }
}

pub fn uninstall<P, S>(base_path: P, name: S) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let base_path = base_path.as_ref();
    let name = name.as_ref();

    let path = base_path.join(name);

    if path.components().count() > base_path.components().count() + 1 {
        Err(InvalidResourceNameError {})?;
    }

    fs_err::remove_dir_all(path)?;

    Ok(())
}
