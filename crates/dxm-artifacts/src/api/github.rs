//! Contains interfaces to interact with the [GitHub API].
//!
//! [github api]: https://docs.github.com/en/rest

use serde::Deserialize;

/// The base URL for GitHub's [Repository REST API](https://docs.github.com/en/rest/repos).
pub const REPOS_API_URL: &str = "https://api.github.com/repos";

/// The media type to accept from GitHub APIs.
/// See [GitHub Docs - Getting started with the REST API](https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api#accept).
pub const ACCEPT_HEADER_VALUE: &str = "application/vnd.github+json";

/// The name of the header for specifying the GitHub API version.
/// See [GitHub Docs - Getting started with the REST API](https://docs.github.com/en/rest/using-the-rest-api/getting-started-with-the-rest-api#x-github-api-version).
pub const API_VERSION_HEADER_NAME: &str = "X-GitHub-Api-Version";

/// The default GitHub API version.
/// See [GitHub Docs - API Versions](https://docs.github.com/en/rest/about-the-rest-api/api-versions).
pub const API_VERSION_HEADER_VALUE: &str = "2022-11-28";

/// The name of FiveM's GitHub Repository.
const FIVEM_REPOSITORY: &str = "citizenfx/fivem";

/// The endpoint for GitHub's [Git Reference API](https://docs.github.com/en/rest/git/refs#get-a-reference).
const GIT_REF_API_ENDPOINT: &str = "/git/ref";

/// The endpoint for GitHub's [Git Tags API](https://docs.github.com/en/rest/git/tags).
const GIT_TAGS_API_ENDPOINT: &str = "/git/tags";

pub fn authorization_token(token: impl AsRef<str>) -> String {
    format!("token {}", token.as_ref())
}

/// Returns GitHub's Git Reference API endpoint for the FiveM repository and the
/// given reference name.
pub fn git_ref_endpoint(ref_name: impl AsRef<str>) -> String {
    format!(
        "/{FIVEM_REPOSITORY}{GIT_REF_API_ENDPOINT}/{}",
        ref_name.as_ref()
    )
}

/// Returns GitHub's Git Tags API endpoint for the FiveM repository and the
/// given tag name.
pub fn git_tags_endpoint(tag_name: impl AsRef<str>) -> String {
    format!(
        "/{FIVEM_REPOSITORY}{GIT_TAGS_API_ENDPOINT}/{}",
        tag_name.as_ref()
    )
}

/// Represents a response from GitHub's Git Reference API.
///
/// Note that the API provides additional data that isn't required for our
/// purposes, and as such isn't included here.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct GitRef {
    /// The object the ref is pointing to.
    pub object: GitObject,
}

/// Represents a response from GitHub's Git Tags API.
///
/// Note that the API provides additional data that isn't required for our
/// purposes, and as such isn't included here.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct GitTag {
    /// The object the tag is pointing to.
    pub object: GitObject,
}

/// Represents a Git Object that a [`GitRef`] or [`GitTag`] can point to.
///
/// Note that the API provides additional data that isn't required for our
/// purposes, and as such isn't included here.
#[derive(Debug, Clone, Deserialize)]
#[non_exhaustive]
pub struct GitObject {
    /// The SHA of the object.
    pub sha: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_git_ref_endpoint() {
        let endpoint = git_ref_endpoint("test");

        assert_eq!(endpoint, "/citizenfx/fivem/git/ref/test");
    }

    #[test]
    fn should_return_git_tags_endpoint() {
        let endpoint = git_tags_endpoint("test");

        assert_eq!(endpoint, "/citizenfx/fivem/git/tags/test");
    }
}
