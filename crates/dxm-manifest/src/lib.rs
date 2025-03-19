//! A crate that contains manifest structures used by dxm.

use std::{error::Error, path::Path};

use serde::{Deserialize, Serialize};

pub mod artifact;
pub mod server;
mod util;

const MANIFEST_NAME: &str = "dxm.toml";

/// The parent manifest structure containing data used by dxm.
#[derive(Default, Serialize, Deserialize)]
pub struct Manifest {
    /// The data about the FXServer installation.
    #[serde(default)]
    pub artifact: artifact::Artifact,

    /// The data about the server.
    #[serde(default)]
    pub server: server::Server,
}

impl Manifest {
    /// Constructs and returns a new `Manifest` instance.
    pub fn new(artifact: artifact::Artifact, server: server::Server) -> Self {
        Self { artifact, server }
    }

    /// Attempts to find a `dxm.toml` file in the given directory,searching
    /// through parent directories as well.
    ///
    /// Returns a new `Manifest` instance if found, or `None` if not.
    pub fn find<P>(dir: P) -> Result<Option<Self>, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let mut dir = dir.as_ref();
        let mut path = dir.join(MANIFEST_NAME);

        while !path.try_exists()? {
            if let Some(parent_dir) = dir.parent() {
                dir = parent_dir;
            } else {
                return Ok(None);
            }

            path = dir.join(MANIFEST_NAME);
        }

        log::debug!("found manifest in {}", dir.display());
        let manifest = Self::read(dir)?;

        Ok(Some(manifest))
    }

    /// Reads a `dxm.toml` file in the given directory, and returns a new
    /// `Manifest` instance.
    pub fn read<P>(dir: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        let path = dir.join(MANIFEST_NAME);

        log::debug!("reading manifest path {}", path.display());

        let contents = fs_err::read_to_string(path)?;
        let manifest = toml::from_str(&contents)?;

        Ok(manifest)
    }

    /// Writes a `dxm.toml` file in the given directory.
    pub fn write<P>(&self, dir: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        let path = dir.join(MANIFEST_NAME);

        log::debug!("writing manifest path {}", path.display());

        let contents = toml::to_string_pretty(self)?;
        fs_err::write(path, contents)?;

        Ok(())
    }
}
