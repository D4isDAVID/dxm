//! Contains structures for the lockfile used by dxm.

use std::{
    collections::BTreeMap,
    error::Error,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub const LOCKFILE_NAME: &str = "dxm-lock.toml";
const LOCKFILE_COMMENT: &str = "\
# THIS IS AN *AUTO-GENERATED* FILE.
# DO *NOT* MODIFY THIS FILE.

";

/// The lockfile structure containing version-locking data used by dxm.
#[derive(Default, Serialize, Deserialize)]
pub struct Lockfile {
    /// The version of the FXServer installation.
    artifact_version: Option<String>,

    /// The download URLs for the third-party FXServer resources.
    #[serde(default)]
    resource_urls: BTreeMap<String, String>,
}

impl Lockfile {
    /// Constructs and returns a new `Lockfile` instance.
    pub fn new(artifact_version: String, resource_urls: BTreeMap<String, String>) -> Self {
        Self {
            artifact_version: Some(artifact_version),
            resource_urls,
        }
    }

    pub fn artifact_version(&self) -> Option<&str> {
        self.artifact_version.as_deref()
    }

    pub fn set_artifact_version<S>(&mut self, artifact_version: S)
    where
        S: Into<String>,
    {
        self.artifact_version = Some(artifact_version.into());
    }

    pub fn get_resource_url<S>(&self, resource_name: S) -> Option<&str>
    where
        S: AsRef<str>,
    {
        self.resource_urls
            .get(resource_name.as_ref())
            .map(|s| s.as_str())
    }

    pub fn set_resource_url<N, S>(&mut self, resource_name: N, resource_url: S)
    where
        N: Into<String>,
        S: Into<String>,
    {
        self.resource_urls
            .insert(resource_name.into(), resource_url.into());
    }

    pub fn remove_resource_url<S>(&mut self, resource_name: S)
    where
        S: AsRef<str>,
    {
        self.resource_urls.remove(resource_name.as_ref());
    }

    /// Reads a `dxm-lock.toml` file in the given directory, and returns a new
    /// `Lockfile` instance.
    pub fn read<P>(dir: P) -> Result<Self, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = Self::dir_manifest(dir);

        log::debug!("reading lockfile path {}", path.display());

        match fs_err::read_to_string(path) {
            Ok(contents) => Ok(toml_edit::de::from_str(&contents)?),
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => Ok(Self::default()),
                _ => Err(err)?,
            },
        }
    }

    /// Writes a `dxm-lock.toml` file in the given directory.
    pub fn write<P>(&self, dir: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = Self::dir_manifest(dir);

        log::debug!("writing lockfile path {}", path.display());

        let contents = toml_edit::ser::to_string(self)?;
        fs_err::write(path, format!("{}{}", LOCKFILE_COMMENT, contents))?;

        Ok(())
    }

    /// Returns the given directory's path joined with the lockfile file name.
    fn dir_manifest<P>(dir: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        dir.join(LOCKFILE_NAME)
    }
}
