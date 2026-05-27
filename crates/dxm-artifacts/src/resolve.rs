//! Contains interfaces for resolving FXServer build versions and fetching
//! FXServer build archives.

use std::path::Path;
use std::{fmt::Debug, io::Cursor};

use bytes::Bytes;
use reqwest::header::{AUTHORIZATION, HeaderValue, InvalidHeaderValue};
use tokio::{fs, io};

use crate::{
    api::{ApiClient, ArtifactsPlatform, ExtractError, authorization_token},
    build::{Build, BuildIssue, BuildVersion},
};

/// Errors that may occur in [`Artifacts`] methods.
#[derive(Debug, thiserror::Error)]
pub enum ArtifactsError {
    /// [`Artifacts::new`] returned an error.
    #[error("failed to initialize artifacts client: {0}")]
    Init(#[source] reqwest::Error),
    /// [`Artifacts::new`] returned an error.
    #[error("failed to insert artifacts header: {0}")]
    Header(#[from] InvalidHeaderValue),
    /// [`Artifacts::resolve`] returned an error when fetching the JG Scripts
    /// Artifacts DB.
    #[error("failed to fetch jg scripts artifacts db: {0}")]
    JgArtifactsDb(#[source] reqwest::Error),
    /// [`Artifacts::resolve`] returned an error when fetching Cfx.re's FXServer
    /// version changelog.
    #[error("failed to fetch cfx version changelog: {0}")]
    CfxServerVersions(#[source] reqwest::Error),
    /// [`Artifacts::resolve`] or [`Artifacts::fetch`] returned an error when
    /// fetching Cfx.re's Artifacts storage.
    #[error("failed to fetch cfx artifacts: {0}")]
    CfxArtifacts(#[source] reqwest::Error),
    /// [`Artifacts::resolve`] returned an error when fetching GitHub's Git
    /// Reference API.
    #[error("failed to fetch github git ref: {0}")]
    GithubGitRef(#[source] reqwest::Error),
    /// [`Artifacts::resolve`] returned an error when fetching GitHub's Git Tags
    /// API.
    #[error("failed to fetch github git tag: {0}")]
    GithubGitTag(#[source] reqwest::Error),
    /// [`Artifacts::download`] returned an error when writing files.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    // [`Artifacts::install`] returned an error when extracting artifacts.
    #[error("failed to extract artifacts: {0}")]
    Extract(#[from] ExtractError),
}

/// Utility type for returning the given type or an [`ArtifactsError`].
pub type ArtifactsResult<T> = Result<T, ArtifactsError>;

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
    pub fn new() -> ArtifactsResult<Self> {
        Ok(Self {
            client: ApiClient::new().map_err(ArtifactsError::Init)?,
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
    pub fn set_github_pat_token(&mut self, token: impl AsRef<str>) -> ArtifactsResult<()> {
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
    pub async fn resolve(&self, version: &BuildVersion) -> ArtifactsResult<Build> {
        let artifacts_db = self
            .client
            .jg_artifacts_db_json()
            .await
            .map_err(ArtifactsError::JgArtifactsDb)?;

        let server_versions = self
            .client
            .cfx_server_versions()
            .await
            .map_err(ArtifactsError::CfxServerVersions)?;

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
            .map_err(ArtifactsError::GithubGitRef)?;
        let tag = self
            .client
            .github_git_tag(tag_ref.object.sha)
            .await
            .map_err(ArtifactsError::GithubGitTag)?;

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
                .map_err(ArtifactsError::CfxArtifacts)?
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

    /// Fetches the given [`Build`] archive for the given [`ArtifactsPlatform`],
    /// and returns it as [`bytes::Bytes`].
    pub async fn fetch(
        &self,
        build: &Build,
        platform: ArtifactsPlatform,
    ) -> ArtifactsResult<Bytes> {
        let bytes = self
            .client
            .cfx_artifacts(platform, build.number(), build.commit_sha())
            .await
            .map_err(ArtifactsError::CfxArtifacts)?;

        Ok(bytes)
    }

    /// Fetches the given [`Build`] archive for the given [`ArtifactsPlatform`],
    /// and writes it in the given path.
    #[tracing::instrument(name = "download_artifacts", skip(self), fields(build = build.number()))]
    pub async fn download(
        &self,
        build: &Build,
        platform: ArtifactsPlatform,
        path: impl AsRef<Path> + Debug,
    ) -> ArtifactsResult<()> {
        let path = path.as_ref();

        let bytes = self.fetch(build, platform).await?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        fs::write(path, bytes).await?;

        Ok(())
    }

    /// Fetches the given [`Build`] archive for the given [`ArtifactsPlatform`],
    /// and extracts it in the given path.
    #[tracing::instrument(name = "install_artifacts", skip(self), fields(build = build.number()))]
    pub async fn install(
        &self,
        build: &Build,
        platform: ArtifactsPlatform,
        path: impl AsRef<Path> + Debug,
    ) -> ArtifactsResult<()> {
        let path = path.as_ref();

        let bytes = self.fetch(build, platform).await?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).await?;
        }

        platform.extract(Cursor::new(bytes), path).await?;

        Ok(())
    }
}
