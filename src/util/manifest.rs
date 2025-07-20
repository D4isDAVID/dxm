use std::{
    error::Error,
    path::{Path, PathBuf},
};

use dxm_manifest::Manifest;

/// Finds and returns the manifest in the given directory path, or returns a new
/// default instance.
pub fn find<P>(path: P) -> Result<(PathBuf, Manifest), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();
    let found_path = Manifest::find(path)?;

    let path = if let Some(found_path) = &found_path {
        found_path.to_owned()
    } else {
        path.to_owned()
    };

    log::debug!("using manifest path {}", path.display());
    let manifest = if found_path.is_none() {
        Manifest::default()
    } else {
        Manifest::read(&path)?
    };

    Ok((path, manifest))
}
