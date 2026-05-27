//! Contains structures representing FXServer builds and their resolvable
//! versions.

use std::{convert::Infallible, str::FromStr};

use crate::api::ArtifactsPlatform;

/// The string identifier for [`BuildVersion::LatestJg`].
const LATEST_JG: &str = "latest-jg";
/// The string identifier for [`BuildVersion::Latest`].
const LATEST: &str = "latest";
/// The string identifier for [`BuildVersion::Recommended`].
const RECOMMENDED: &str = "recommended";

/// Versions of FXServer that are possible to resolve to a build number.
///
/// The Cfx.re [Changelogs API] provides additional `optional` and `critical`
/// versions, but they are excluded from this list as they don't get updated.
///
/// [changelogs api]: https://changelogs-live.fivem.net/api/changelog/versions/win32/server
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum BuildVersion {
    /// A specific FXServer build number with no resolving needed.
    BuildNumber(String),
    /// The latest artifacts build for FXServer with no reported issues on the
    /// JG Scripts [Artifacts DB](https://artifacts.jgscripts.com/). The
    /// resulting build *should* be stable.
    #[default]
    LatestJg,
    /// The latest artifacts build available for FXServer. The resulting build
    /// *might* be stable.
    Latest,
    /// The recommended[^1] artifacts build available for FXServer. The
    /// resulting build will be stable.
    ///
    /// [^1]: "recommended" by the Cfx.re Changelogs API, however this version
    /// can get out of date quickly. Please check before using.
    Recommended,
}

impl std::fmt::Display for BuildVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            BuildVersion::BuildNumber(number) => number,
            BuildVersion::LatestJg => LATEST_JG,
            BuildVersion::Latest => LATEST,
            BuildVersion::Recommended => RECOMMENDED,
        })
    }
}

impl FromStr for BuildVersion {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            LATEST_JG => Self::LatestJg,
            LATEST => Self::Latest,
            RECOMMENDED => Self::Recommended,
            _ => Self::BuildNumber(s.to_owned()),
        })
    }
}

/// A resolved FXServer build that can have zero or more issues.
#[derive(Debug, Clone)]
pub struct Build {
    /// The number tag associated to this FXServer build.
    number: String,
    /// The SHA of the commit FXServer was built from.
    commit_sha: String,
    /// The issues this FXServer build has.
    issues: Vec<BuildIssue>,
}

impl Build {
    /// Creates and returns a new [`Build`] with the given data.
    pub fn new(number: String, commit_sha: String, issues: Vec<BuildIssue>) -> Self {
        Self {
            number,
            commit_sha,
            issues,
        }
    }

    /// Returns the number tag associated to this FXServer build.
    pub fn number(&self) -> &str {
        &self.number
    }

    /// Returns the SHA of the commit this FXServer build was built from.
    pub fn commit_sha(&self) -> &str {
        &self.commit_sha
    }

    /// Returns a list of issues this FXServer build has.
    pub fn issues(&self) -> impl Iterator<Item = &BuildIssue> {
        self.issues.iter()
    }
}

/// Issues that a [`Build`] can contain.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum BuildIssue {
    /// The FXServer build is not supported according to Cfx.re's
    /// [Changelogs API](https://changelogs-live.fivem.net/api/changelog/versions/win32/server).
    Unsupported,
    /// The FXServer build has an issue reported on the JG Scripts
    /// [Artifacts DB](https://artifacts.jgscripts.com).
    JgReport(String),
    /// The FXServer build is unavailable for a specific platform.
    UnsupportedPlatform(ArtifactsPlatform),
}

impl std::fmt::Display for BuildIssue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildIssue::Unsupported => write!(f, "this build is out of support"),
            BuildIssue::JgReport(issue) => {
                write!(f, "reported on the JG Scripts Artifacts DB: {issue}")
            }
            BuildIssue::UnsupportedPlatform(platform) => {
                write!(f, "this build isn't available for {platform:?}")
            }
        }
    }
}
