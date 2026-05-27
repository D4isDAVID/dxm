//! Contains interfaces to interact with the JG Scripts [Artifacts DB].
//!
//! [artifacts db]: https://artifacts.jgscripts.com

use std::collections::HashMap;

use serde::Deserialize;

/// The URL for the JG Scripts [Artifacts DB](https://artifacts.jgscripts.com).
pub const BASE_URL: &str = "https://artifacts.jgscripts.com";

/// The JSON API endpoint for the JG Scripts [Artifacts DB](https://artifacts.jgscripts.com).
pub const JSON_ENDPOINT: &str = "/json";

/// Represents a result from [`ARTIFACTS_DB_URL`].
///
/// Note that the API also provides download links for Windows and Linux which
/// are not included here, as they are resolved later.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct JgArtifacts {
    /// The latest artifacts build for FXServer with no reported issues.
    #[serde(rename = "recommendedArtifact")]
    pub recommended_artifact: String,
    /// FXServer build numbers or ranges mapped to their reported issues.
    /// Build ranges will look something like this: `"26261-27715"`.
    #[serde(rename = "brokenArtifacts")]
    pub broken_artifacts: HashMap<String, String>,
}
