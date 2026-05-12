use std::{error::Error, io::Write, path::Path};

use reqwest::blocking::Client;
use tempfile::{NamedTempFile, TempDir};
use zip::{ZipArchive, read::root_dir_common_filter};

pub fn download<S, P, N>(
    client: &Client,
    url: S,
    path: P,
    nested_path: N,
) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
    P: AsRef<Path>,
    N: AsRef<Path>,
{
    let url = url.as_ref();
    let path = path.as_ref();
    let nested_path = nested_path.as_ref();

    let mut file = NamedTempFile::with_suffix("dxm-resource-archive")?;
    let bytes = client.get(url).send()?.error_for_status()?.bytes()?;
    file.write_all(&bytes)?;

    let dir = TempDir::with_suffix("dxm-resource")?;
    log::trace!("extracting resource");
    ZipArchive::new(file.reopen()?)?.extract_unwrapped_root_dir(&dir, root_dir_common_filter)?;

    log::debug!("moving contents into {}", path.display());
    crate::move_dir_contents(dir.path().join(nested_path), path)?;

    Ok(())
}
