//! A crate that contains manifest structures used by dxm.

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::util::add_and_fill_missing_table;

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

    /// Attempts to find a `dxm.toml` file in the given directory, searching
    /// through parent directories as well.
    ///
    /// Returns the manifest's directory if found, or `None` if not.
    pub fn find<P>(dir: P) -> Result<Option<PathBuf>, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let full_dir = fs_err::canonicalize(dir.as_ref())?;

        let mut dir = full_dir.as_path();
        let mut path = Self::dir_manifest(dir);

        while !path.try_exists()? {
            if let Some(parent_dir) = dir.parent() {
                dir = parent_dir;
            } else {
                log::debug!(
                    "could not find manifest in {} or its parents",
                    full_dir.display()
                );

                return Ok(None);
            }

            path = Self::dir_manifest(dir);
        }

        log::debug!("found manifest in {}", dir.display());

        Ok(Some(dir.to_path_buf()))
    }

    /// Reads a `dxm.toml` file in the given directory, and returns a new
    /// `Manifest` instance.
    pub fn read<P>(dir: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = Self::dir_manifest(dir);

        log::debug!("reading manifest path {}", path.display());

        let contents = fs_err::read_to_string(path)?;
        let manifest = toml_edit::de::from_str(&contents)?;

        Ok(manifest)
    }

    /// Writes a `dxm.toml` file in the given directory.
    pub fn write<P>(&self, dir: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = Self::dir_manifest(dir);

        let mut document = match fs_err::read_to_string(&path) {
            Ok(content) => {
                log::debug!("parsing manifest path {}", path.display());

                content.parse()?
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => {
                    log::trace!("creating new manifest file");

                    toml_edit::DocumentMut::new()
                }
                _ => Err(err)?,
            },
        };

        log::debug!("writing manifest path {}", path.display());

        self.fill_document(&mut document);
        fs_err::write(path, document.to_string())?;

        Ok(())
    }

    /// Fills out the manifest inside the given TOML document.
    fn fill_document(&self, document: &mut toml_edit::DocumentMut) {
        add_and_fill_missing_table(document, "artifact", |i| self.artifact.fill_toml_item(i));
        add_and_fill_missing_table(document, "server", |i| self.server.fill_toml_item(i));
    }

    /// Returns the given directory's path joined with the manifest file name.
    fn dir_manifest<P>(dir: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        dir.join(MANIFEST_NAME)
    }
}
