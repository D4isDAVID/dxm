//! Contains interfaces for resolving FXServer build versions and fetching
//! FXServer build archives.

use std::fmt::Debug;

use bytes::Bytes;
use futures_core::stream::BoxStream;
use reqwest::header::{AUTHORIZATION, HeaderValue};

use crate::{
    api::{ApiClient, ArtifactsPlatform, authorization_token},
    build::{Build, BuildIssue, BuildVersion},
    error::FetchError,
};

/// The main interface for resolving & downloading FXServer builds.
#[derive(Debug, Clone)]
pub struct Artifacts {
    /// The client to download the build with.
    client: ApiClient,
}

impl Artifacts {
    /// Creates and returns a new [`Artifacts`] instance with an empty
    /// [`reqwest::Client`] and default API URLs.
    ///
    /// See [`Self::with_client`].
    pub fn new() -> crate::Result<Self> {
        Ok(Self {
            client: ApiClient::new().map_err(crate::Error::Init)?,
        })
    }

    /// Creates and returns a new [`Artifacts`] instance with the given
    /// [`reqwest::Client`] and default API URLs.
    ///
    /// See [`Self::with_urls`].
    pub fn with_client(client: reqwest::Client) -> Self {
        Self {
            client: ApiClient::with_client(client),
        }
    }

    /// Creates and returns a new [`Artifacts`] instance with the given
    /// [`reqwest::Client`] and the given API URLs.
    ///
    /// For the GitHub Repositories API, it inserts default headers for `Accept`
    /// and `X-GitHub-Api-Version`.
    pub fn with_urls(
        client: reqwest::Client,
        jg_artifacts_db_base_url: impl Into<String>,
        cfx_changelogs_api_base_url: impl Into<String>,
        cfx_artifacts_base_url: impl Into<String>,
        github_repos_api_base_url: impl Into<String>,
    ) -> Self {
        Self {
            client: ApiClient::with_urls(
                client,
                jg_artifacts_db_base_url,
                cfx_changelogs_api_base_url,
                cfx_artifacts_base_url,
                github_repos_api_base_url,
            ),
        }
    }

    /// Runs [`reqwest::header::HeaderMap::insert`] with the given data for
    /// GitHub's Repository REST API.
    pub fn set_github_pat_token(&mut self, token: impl AsRef<str>) -> crate::Result<()> {
        self.client.github_repos_api.headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&authorization_token(token))?,
        );

        Ok(())
    }

    /// Resolves the FXServer build number from the given [`BuildVersion`],
    /// resolves any possible [`BuildIssue`]s, wraps them in a new
    /// [`Build`] and returns it.
    #[tracing::instrument(name = "resolve_artifacts", skip(self))]
    pub async fn resolve(&self, version: &BuildVersion) -> crate::Result<Build> {
        let artifacts_db = self
            .client
            .jg_artifacts_db_json()
            .await
            .map_err(FetchError::JgArtifactsDb)?;

        let server_versions = self
            .client
            .cfx_server_versions()
            .await
            .map_err(FetchError::CfxServerVersions)?;

        let number = match version {
            BuildVersion::BuildNumber(build) => build.clone(),
            BuildVersion::LatestJg => artifacts_db.recommended_artifact,
            BuildVersion::Latest => server_versions.latest,
            BuildVersion::Recommended => server_versions.recommended,
        };
        tracing::debug!(number = number, "resolved build number",);

        let tag_ref = format!("tags/v1.0.0.{number}");
        let tag_ref = self
            .client
            .github_git_ref(tag_ref)
            .await
            .map_err(FetchError::GithubGitRef)?;
        let tag = self
            .client
            .github_git_tag(tag_ref.object.sha)
            .await
            .map_err(FetchError::GithubGitTag)?;

        let commit_sha = tag.object.sha;
        tracing::debug!(commit_sha = commit_sha, "resolved build commit");

        let mut issues = Vec::new();

        let support_policy = server_versions.support_policy.get(&number);
        if support_policy.is_none_or(|s| s < &chrono::Utc::now()) {
            issues.push(BuildIssue::Unsupported);

            tracing::debug!(number = number, "found build is unsupported");
        }

        for (key, value) in artifacts_db.broken_artifacts.iter() {
            if key == &number
                || key
                    .split_once('-')
                    .and_then(|(a, b)| Some((a.parse::<u32>().ok()?, b.parse::<u32>().ok()?)))
                    .is_some_and(|(from, to)| {
                        number.parse::<u32>().is_ok_and(|n| n >= from && n <= to)
                    })
            {
                issues.push(BuildIssue::JgReport(value.clone()));

                tracing::debug!(
                    number = number,
                    issue = value,
                    "found reported issue for build"
                );
            }
        }

        for platform in ArtifactsPlatform::iter() {
            if !self
                .client
                .check_cfx_artifacts(platform, &number, &commit_sha)
                .await
                .map_err(FetchError::CfxArtifacts)?
            {
                issues.push(BuildIssue::UnsupportedPlatform(platform));

                tracing::debug!(
                    number = number,
                    platform = ?platform,
                    "found build is unavailable for platform"
                );
            }
        }

        Ok(Build::new(number, commit_sha, issues))
    }

    /// Fetches the given [`Build`] archive for the compiled platform, and
    /// returns the content bytes stream, and the content length if available.
    pub async fn fetch(
        &self,
        build: &Build,
    ) -> crate::Result<(BoxStream<'static, reqwest::Result<Bytes>>, Option<u64>)> {
        self.fetch_platform(build, ArtifactsPlatform::default())
            .await
    }

    /// Fetches the given [`Build`] archive for the given [`ArtifactsPlatform`],
    /// and returns the content bytes stream, and the content length if
    /// available.
    pub async fn fetch_platform(
        &self,
        build: &Build,
        platform: ArtifactsPlatform,
    ) -> crate::Result<(BoxStream<'static, reqwest::Result<Bytes>>, Option<u64>)> {
        let result = self
            .client
            .cfx_artifacts(platform, build.number(), build.commit_sha())
            .await
            .map_err(FetchError::CfxArtifacts)?;

        Ok(result)
    }
}
