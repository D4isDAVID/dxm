//! Contains interfaces to interact with Cfx.re's [Changelogs API].
//!
//! [changelogs api]: https://changelogs-live.fivem.net/api/changelog/versions/win32/server

use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer};

/// The base URL for Cfx.re's Changelogs API.
pub const API_BASE_URL: &str = "https://changelogs-live.fivem.net/api";

/// The endpoint for the Windows FXServer version changelog in Cfx.re's
/// Changelogs API.
pub const WINDOWS_SERVER_ENDPOINT: &str = "/changelog/versions/win32/server";

/// Represents a response from Cfx.re's Changelogs API.
///
/// Note that the API also provides download links which are not included here,
/// as they are resolved later. Additional versions such as `optional` and
/// `critical` are not included as they don't get updated.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct ServerVersions {
    /// The number tag associated to the latest available FXServer build.
    pub latest: String,
    /// The number tag associated to the recommended[^1] available FXServer
    /// build.
    ///
    /// [^1]: "recommended" by Cfx.re's Changelogs API, however this version
    /// can get out of date quickly. Please check before using.
    pub recommended: String,
    /// FXServer build numbers mapped to their final support date. If a build
    /// number is missing from here, it's definitely out of support.
    #[serde(deserialize_with = "deserialize_support_policy")]
    pub support_policy: HashMap<String, DateTime<Utc>>,
}

/// Adds the Z prefix to support date timestamps without it, then parses the
/// timestamps with [`chrono::DateTime::parse_from_rfc3339`].
///
/// This is needed as `latest` and `recommended` versions use the
/// `9999-12-31T23:59:59.9999999` timestamp, which does not include a timestamp.
fn deserialize_support_policy<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    HashMap::<String, String>::deserialize(deserializer)?
        .into_iter()
        .map(|(k, mut v)| {
            if !v.ends_with('Z') {
                v.push('Z');
            }

            let datetime = DateTime::parse_from_rfc3339(&v)
                .map_err(serde::de::Error::custom)?
                .with_timezone(&Utc);

            Ok((k, datetime))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_deserialize_support_policy() {
        let json = r#"{ "latest": "2", "recommended": "1", "support_policy": { "2": "9999-12-31T23:59:59.9999999", "1": "2026-01-01T00:00:00Z" } }"#;
        let server_versions: ServerVersions = serde_json::from_str(json).unwrap();

        let utc = server_versions.support_policy["1"];
        let lts = server_versions.support_policy["2"];

        assert_eq!(
            utc,
            DateTime::parse_from_rfc3339("2026-01-01T00:00:00Z").unwrap()
        );
        assert_eq!(
            lts,
            DateTime::parse_from_rfc3339("9999-12-31T23:59:59.9999999Z").unwrap()
        );
    }
}
