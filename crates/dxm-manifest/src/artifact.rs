//! Contains structures that represent FXServer installation data.

use std::{
    path::{Path, PathBuf, StripPrefixError},
    str::FromStr,
    sync::LazyLock,
};

pub use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use serde::{Deserialize, Serialize};

use crate::util::relative_path;

static DEFAULT_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("artifact"));
static DEFAULT_CHANNEL: LazyLock<String> = LazyLock::new(|| ArtifactsChannel::LatestJg.to_string());

/// Represents a dxm-managed FXServer installation.
#[derive(Serialize, Deserialize)]
pub struct Artifact {
    /// The FXServer installation path.
    path: Option<PathBuf>,
    /// The FXServer artifact version.
    version: Option<String>,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            path: Some(DEFAULT_PATH.to_path_buf()),
            version: Some(DEFAULT_CHANNEL.to_string()),
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
        self.path = Some(relative_path(manifest_path, path)?);

        Ok(())
    }

    /// Returns the installation's path relative to the manifest file path.
    fn relative_path(&self) -> &PathBuf {
        self.path.as_ref().unwrap_or(&*DEFAULT_PATH)
    }

    /// Returns the installation's path appended to the given manifest file path.
    pub fn path<P>(&self, manifest_path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        manifest_path.as_ref().join(self.relative_path())
    }

    /// Returns the installation's FXServer path appended to the given manifest file path.
    pub fn exe<P>(&self, manifest_path: P, platform: ArtifactsPlatform) -> PathBuf
    where
        P: AsRef<Path>,
    {
        self.path(manifest_path).join(platform.exe_name())
    }

    /// Sets the installation's version.
    pub fn set_version<S>(&mut self, version: S)
    where
        S: Into<String>,
    {
        self.version = Some(version.into());
    }

    /// Returns the installation's version.
    pub fn version(&self) -> &str {
        self.version.as_deref().unwrap_or(DEFAULT_CHANNEL.as_str())
    }

    /// Returns the installation's channel if the version is a channel, None
    /// otherwise.
    pub fn channel(&self) -> Option<ArtifactsChannel> {
        ArtifactsChannel::from_str(self.version()).ok()
    }

    /// Fills out information about the installation inside the given TOML document.
    pub fn fill_toml_item(&self, item: &mut toml_edit::Item) {
        item["path"] = toml_edit::value(self.relative_path().to_string_lossy().into_owned());
        item["version"] = toml_edit::value(self.version());
    }
}
