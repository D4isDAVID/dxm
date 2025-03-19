//! Contains code for accessing the FXServer version changelog.

pub use channel::ArtifactsChannel;
pub use platform::ArtifactsPlatform;
use reqwest::blocking::Client;
use serde::Deserialize;

mod channel;
mod platform;

const CFX_SERVER_VERSIONS_API_URL: &str =
    "https://changelogs-live.fivem.net/api/changelog/versions/{platform}/server";

/// Represents the FXServer version changelog.
///
/// Does not support `ArtifactsChannel::LatestJg`.
/// Use `dxm_artifacts::jg` instead.
#[derive(Deserialize)]
pub struct ServerVersions {
    critical: String,
    recommended: String,
    optional: String,
    latest: String,

    critical_txadmin: String,
    recommended_txadmin: String,
    optional_txadmin: String,
    latest_txadmin: String,
}

impl ServerVersions {
    /// Returns the FXServer version for the given update channel.
    ///
    /// Panics if given `LatestJg` - use `dxm_artifacts::jg` instead.
    pub fn version(&self, channel: &ArtifactsChannel) -> &str {
        match channel {
            ArtifactsChannel::Critical => &self.critical,
            ArtifactsChannel::Recommended => &self.recommended,
            ArtifactsChannel::Optional => &self.optional,
            ArtifactsChannel::Latest => &self.latest,
            ArtifactsChannel::LatestJg => {
                panic!("received unexpected LatestJg in cfx server versions")
            }
        }
    }

    /// Returns the txAdmin version for the given update channel.
    ///
    /// Panics if given `LatestJg` - use `dxm_artifacts::jg` instead.
    pub fn txadmin(&self, channel: &ArtifactsChannel) -> &str {
        match channel {
            ArtifactsChannel::Critical => &self.critical_txadmin,
            ArtifactsChannel::Recommended => &self.recommended_txadmin,
            ArtifactsChannel::Optional => &self.optional_txadmin,
            ArtifactsChannel::Latest => &self.latest_txadmin,
            ArtifactsChannel::LatestJg => {
                panic!("received unexpected LatestJg in cfx server versions")
            }
        }
    }

    /// Returns a string containing information for the given update channel.
    pub fn alias_display(&self, alias: &ArtifactsChannel) -> String {
        format!(
            "{}\twith txAdmin v{}",
            self.version(alias),
            self.txadmin(alias)
        )
    }
}

/// Fetches and returns the FXServer version changelog.
pub fn versions(client: &Client, platform: &ArtifactsPlatform) -> reqwest::Result<ServerVersions> {
    log::trace!("getting artifacts versions");

    let url = changelogs_url(platform);
    let resp = client.get(url).send()?;

    resp.json::<ServerVersions>()
}

/// Returns the changelog URL for the given platform.
fn changelogs_url(platform: &ArtifactsPlatform) -> String {
    CFX_SERVER_VERSIONS_API_URL.replace("{platform}", platform.changelogs_name())
}
