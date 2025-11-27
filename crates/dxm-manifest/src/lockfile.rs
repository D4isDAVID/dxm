//! Contains structures for the lockfile used by dxm.

use std::{
    collections::HashMap,
    error::Error,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

const LOCKFILE_NAME: &str = "dxm-lock.toml";
const LOCKFILE_COMMENT: &str = "\
# THIS IS AN *AUTO-GENERATED* FILE.
# DO *NOT* MODIFY THIS FILE.

";

/// The lockfile structure containing version-locking data used by dxm.
#[derive(Default, Serialize, Deserialize)]
pub struct Lockfile {
    /// The download URL for the FXServer build.
    artifact_url: Option<String>,

    /// The download URLs for the third-party FXServer resources.
    #[serde(default)]
    resource_urls: HashMap<String, String>,
}

impl Lockfile {
    /// Constructs and returns a new `Lockfile` instance.
    pub fn new(artifact_url: String, resource_urls: HashMap<String, String>) -> Self {
        Self {
            artifact_url: Some(artifact_url),
            resource_urls,
        }
    }

    pub fn artifact_url(&self) -> Option<&str> {
        self.artifact_url.as_deref()
    }

    pub fn set_artifact_url<S>(&mut self, artifact_url: S)
    where
        S: Into<String>,
    {
        self.artifact_url = Some(artifact_url.into());
    }

    pub fn resource_urls(&self) -> &HashMap<String, String> {
        &self.resource_urls
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

        log::debug!("reading lockfile path {}", path.display());

        let contents = toml_edit::ser::to_string(self)?;
        fs_err::write(path, format!("{}{}", LOCKFILE_COMMENT, contents))?;

        Ok(())
    }

    pub fn write_artifact_url<P, S>(dir: P, artifact_url: S) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let dir = dir.as_ref();
        let artifact_url = artifact_url.as_ref();

        let mut doc = Self::read(dir)?;
        doc.set_artifact_url(artifact_url);
        doc.write(dir)
    }

    pub fn write_resource_url<P, N, S>(
        dir: P,
        resource_name: N,
        resource_url: S,
    ) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
        N: AsRef<str>,
        S: AsRef<str>,
    {
        let dir = dir.as_ref();
        let resource_name = resource_name.as_ref();
        let resource_url = resource_url.as_ref();

        let mut doc = Self::read(dir)?;
        doc.set_resource_url(resource_name, resource_url);
        doc.write(dir)
    }

    pub fn unwrite_resource_url<P, S>(dir: P, resource_name: S) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
    {
        let dir = dir.as_ref();
        let resource_name = resource_name.as_ref();

        let mut doc = Self::read(dir)?;
        doc.remove_resource_url(resource_name);
        doc.write(dir)
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
