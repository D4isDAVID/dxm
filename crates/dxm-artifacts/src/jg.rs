//! Contains code for accessing JGScripts' Artifacts DB.

use reqwest::blocking::Client;
use serde::Deserialize;

const JGSCRIPTS_ARTIFACTS_API_URL: &str = "https://artifacts.jgscripts.com/json";

/// Represents the JGScripts Artifacts DB.
#[derive(Deserialize)]
pub struct Artifacts {
    #[serde(rename = "recommendedArtifact")]
    recommended_artifact: String,
}

impl Artifacts {
    /// Returns the recommended version.
    pub fn version(&self) -> &str {
        &self.recommended_artifact
    }

    /// Returns a string containing information about the recommended version.
    pub fn alias_display(&self) -> String {
        self.version().to_string()
    }
}

/// Fetches and returns the JGScripts Artifacts DB.
pub fn artifacts(client: &Client) -> reqwest::Result<Artifacts> {
    log::trace!("getting artifacts database");

    let resp = client.get(JGSCRIPTS_ARTIFACTS_API_URL).send()?;

    resp.json::<Artifacts>()
}
