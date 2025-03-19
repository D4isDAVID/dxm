use std::{error::Error, path::Path};

use dxm_manifest::Manifest;

/// Finds and returns the manifest in the given directory path, or returns a new
/// default instance.
pub fn find<P>(path: P) -> Result<Manifest, Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    log::debug!("using manifest path {}", path.display());
    let manifest = Manifest::find(path)?.unwrap_or_else(Manifest::default);

    Ok(manifest)
}
