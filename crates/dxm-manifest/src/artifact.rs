//! Contains structures that represent FXServer installation data.

use std::{
    path::{Path, PathBuf, StripPrefixError},
    str::FromStr,
    sync::LazyLock,
};

pub use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use serde::{Deserialize, Serialize};

use crate::{
    resource::Resource,
    util::{add_and_fill_inline_table, relative_path},
};

pub const SYSTEM_RESOURCES: &str = "system_resources";
pub const TXADMIN_MONITOR: &str = ".dxm-monitor-tx";

static DEFAULT_PATH: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("artifact"));
static DEFAULT_CHANNEL: LazyLock<String> = LazyLock::new(|| ArtifactsChannel::LatestJg.to_string());

/// Represents a dxm-managed FXServer installation.
#[derive(Serialize, Deserialize)]
pub struct Artifact {
    /// The FXServer installation path.
    path: Option<PathBuf>,
    /// The FXServer artifact version.
    version: Option<String>,
    /// An optional third-party FXServer monitor.
    monitor: Option<Resource>,
}

impl Default for Artifact {
    fn default() -> Self {
        Self {
            path: Some(DEFAULT_PATH.to_path_buf()),
            version: Some(DEFAULT_CHANNEL.to_string()),
            monitor: None,
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
    pub fn exe<P>(&self, manifest_path: P, platform: &ArtifactsPlatform) -> PathBuf
    where
        P: AsRef<Path>,
    {
        self.path(manifest_path).join(platform.exe_name())
    }

    /// Returns the installation's system_resources path appended to the given manifest file path.
    pub fn system_resources<P>(&self, manifest_path: P, platform: &ArtifactsPlatform) -> PathBuf
    where
        P: AsRef<Path>,
    {
        platform
            .citizen_dir(self.path(manifest_path))
            .join(SYSTEM_RESOURCES)
    }

    /// Returns the installation's tx monitor path appended to the given manifest file path.
    pub fn tx_monitor<P>(&self, manifest_path: P, platform: &ArtifactsPlatform) -> PathBuf
    where
        P: AsRef<Path>,
    {
        platform
            .citizen_dir(self.path(manifest_path))
            .join(TXADMIN_MONITOR)
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

    /// Sets the installation's FXServer monitor.
    pub fn set_monitor(&mut self, monitor: Resource) {
        self.monitor = Some(monitor);
    }

    /// Remove the installation's FXServer monitor.
    pub fn remove_monitor(&mut self) {
        self.monitor = None;
    }

    /// Returns the installation's FXServer monitor.
    pub fn monitor(&self) -> Option<&Resource> {
        self.monitor.as_ref()
    }

    /// Fills out information about the installation inside the given TOML document.
    pub fn fill_toml_item(&self, item: &mut toml_edit::Item) {
        item["path"] = toml_edit::value(self.relative_path().to_string_lossy().into_owned());
        item["version"] = toml_edit::value(self.version());

        if let Some(monitor) = self.monitor() {
            add_and_fill_inline_table(item, "monitor", |i| monitor.fill_toml_item(i));
        } else {
            if let Some(t) = item.as_table_like_mut() {
                t.remove("monitor");
            }
        }
    }
}
