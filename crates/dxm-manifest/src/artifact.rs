//! Contains structures that represent FXServer installation data.

use std::path::{Path, PathBuf, StripPrefixError};

pub use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use serde::{Deserialize, Serialize};

use crate::util::relative_path;

/// Represents a dxm-managed FXServer installation.
#[derive(Serialize, Deserialize)]
pub struct Artifact {
    /// The FXServer installation path.
    ///
    /// Default: `artifact`
    path: PathBuf,
    /// The FXServer artifact version.
    ///
    /// Default: `""`
    pub version: String,
    /// The FXServer update channel such as recommended, latest, etc.
    ///
    /// Default `LatestJg`
    pub channel: ArtifactsChannel,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            path: PathBuf::from("artifact"),
            version: "".to_owned(),
            channel: ArtifactsChannel::LatestJg,
        }
    }
}

impl Artifact {
    /// Sets the installation's path relative to the given manifest file path.
    pub fn set_path<M, P>(&mut self, manifest_path: M, path: P) -> Result<(), StripPrefixError>
    where
        M: AsRef<Path>,
        P: AsRef<Path>,
    {
        self.path = relative_path(manifest_path, path)?;

        Ok(())
    }

    /// Returns the installation's path appended to the given manifest file path.
    pub fn path<P>(&self, manifest_path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        manifest_path.as_ref().join(&self.path)
    }

    /// Returns the installation's FXServer path appended to the given manifest file path.
    pub fn exe<P>(&self, manifest_path: P, platform: ArtifactsPlatform) -> PathBuf
    where
        P: AsRef<Path>,
    {
        self.path(manifest_path).join(platform.exe_name())
    }
}
