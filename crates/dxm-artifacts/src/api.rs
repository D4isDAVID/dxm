//! Contains interfaces for interacting with APIs needed for resolving and
//! downloading FXServer builds.

use bytes::Bytes;
use reqwest::{
    Client, ClientBuilder, Method, Response, StatusCode,
    header::{self, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::de::DeserializeOwned;

pub use crate::api::{
    cfx::artifacts::{ArtifactsPlatform, ExtractError},
    github::authorization_token,
};
use crate::api::{
    cfx::{
        artifacts::{self},
        changelogs::{self, ServerVersions},
    },
    github::{GitRef, GitTag, git_ref_endpoint, git_tags_endpoint},
};

mod cfx;
mod github;
mod jg;

const USER_AGENT_VALUE: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

/// Stores data to make requests with.
#[derive(Debug, Clone)]
pub struct ApiData {
    /// The base URL to make requests to.
    pub base_url: String,
    /// The headers that requests will be made with.
    pub headers: HeaderMap<HeaderValue>,
}

impl ApiData {
    /// Creates and returns a new [`ApiData`] instance with the given base URL
    /// and empty headers.
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            headers: HeaderMap::new(),
        }
    }
}

/// Wrapper for [`reqwest::Client`] to interact with web APIs needed for
/// resolving and downloading FXServer builds.
#[derive(Debug, Clone)]
pub struct ApiClient {
    /// The client with which web requests will be made.
    client: Client,
    /// The data for the JG Scripts [Artifacts DB](https://artifacts.jgscripts.com).
    pub jg_artifacts_db: ApiData,
    /// The data for Cfx.re's Changelogs API.
    pub cfx_changelogs_api: ApiData,
    /// The data for Cfx.re's Runtime Artifacts storage.
    pub cfx_artifacts: ApiData,
    /// The data for GitHub's [Repository REST API](https://docs.github.com/en/rest/repos).
    pub github_repos_api: ApiData,
}

impl ApiClient {
    /// Creates and returns a new [`ApiClient`] with an empty [`Client`] and
    /// default API URLs.
    ///
    /// See [`Self::with_client`].
    pub fn new() -> reqwest::Result<Self> {
        Ok(Self::with_client(ClientBuilder::new().build()?))
    }

    /// Creates and returns a new [`ApiClient`] with the given [`Client`] and
    /// default API URLs.
    ///
    /// See [`Self::with_urls`].
    pub fn with_client(client: Client) -> Self {
        Self::with_urls(
            client,
            jg::BASE_URL,
            changelogs::API_BASE_URL,
            artifacts::BASE_URL,
            github::REPOS_API_URL,
        )
    }

    /// Creates and returns a new [`ApiClient`] with the given [`Client`] and
    /// the given API URLs.
    ///
    /// For the GitHub Repositories API, it inserts headers for `Accept` and
    /// `X-GitHub-Api-Version`.
    pub fn with_urls(
        client: Client,
        jg_artifacts_db_base_url: impl Into<String>,
        cfx_changelogs_api_base_url: impl Into<String>,
        cfx_artifacts_base_url: impl Into<String>,
        github_repos_api_base_url: impl Into<String>,
    ) -> Self {
        let mut github_repos_api = ApiData::new(github_repos_api_base_url);

        github_repos_api.headers.insert(
            header::ACCEPT,
            HeaderValue::from_static(github::ACCEPT_HEADER_VALUE),
        );
        github_repos_api.headers.insert(
            github::API_VERSION_HEADER_NAME,
            HeaderValue::from_static(github::API_VERSION_HEADER_VALUE),
        );

        Self {
            client,
            jg_artifacts_db: ApiData::new(jg_artifacts_db_base_url),
            cfx_changelogs_api: ApiData::new(cfx_changelogs_api_base_url),
            cfx_artifacts: ApiData::new(cfx_artifacts_base_url),
            github_repos_api,
        }
    }

    /// Makes a `GET` request to the JG Scripts Artifacts DB JSON API, parses
    /// the results as JSON and returns the result.
    #[tracing::instrument(name = "fetch_artifacts_db", skip(self))]
    pub async fn jg_artifacts_db_json(&self) -> reqwest::Result<jg::JgArtifacts> {
        self.get_json(&self.jg_artifacts_db, jg::JSON_ENDPOINT)
            .await
    }

    /// Makes a `GET` request to Cfx.re's Server Versions Changelog API, parses
    /// the results as JSON and returns the result.
    #[tracing::instrument(name = "fetch_changelogs", skip(self))]
    pub async fn cfx_server_versions(&self) -> reqwest::Result<ServerVersions> {
        // use windows endpoint as the source of truth as it is prioritized over linux
        self.get_json(
            &self.cfx_changelogs_api,
            changelogs::WINDOWS_SERVER_ENDPOINT,
        )
        .await
    }

    /// Makes a `GET` request to Cfx.re's Artifacts storage, and returns the
    /// results as [`Bytes`]`.
    #[tracing::instrument(name = "fetch_artifacts", skip(self, commit_sha), fields(number = number.as_ref()))]
    pub async fn cfx_artifacts(
        &self,
        platform: ArtifactsPlatform,
        number: impl AsRef<str>,
        commit_sha: impl AsRef<str>,
    ) -> reqwest::Result<Bytes> {
        self.get(
            &self.cfx_artifacts,
            artifacts::fxserver_endpoint(platform, number, commit_sha),
        )
        .await?
        .bytes()
        .await
    }

    /// Makes a `HEAD` request to Cfx.re's Artifacts storage, and returns
    /// whether it exists.
    #[tracing::instrument(name = "check_artifacts", skip(self, commit_sha), fields(number = number.as_ref()))]
    pub async fn check_cfx_artifacts(
        &self,
        platform: ArtifactsPlatform,
        number: impl AsRef<str>,
        commit_sha: impl AsRef<str>,
    ) -> reqwest::Result<bool> {
        let response = self
            .head(
                &self.cfx_artifacts,
                artifacts::fxserver_endpoint(platform, number, commit_sha),
            )
            .await?;

        let not_found = response.status() == StatusCode::NOT_FOUND;
        if !not_found {
            response.error_for_status()?;
        }

        Ok(!not_found)
    }

    /// Makes a `GET` request to GitHub's Git Reference API, parses the results
    /// as JSON and returns the result.
    #[tracing::instrument(name = "fetch_ref", skip(self), fields(ref_name = ref_name.as_ref()))]
    pub async fn github_git_ref(&self, ref_name: impl AsRef<str>) -> reqwest::Result<GitRef> {
        self.get_json(&self.github_repos_api, git_ref_endpoint(ref_name))
            .await
    }

    /// Makes a `GET` request to GitHub's Git Tags API, parses the results as
    /// JSON and returns the result.
    #[tracing::instrument(name = "fetch_tag", skip(self), fields(tag_sha = tag_sha.as_ref()))]
    pub async fn github_git_tag(&self, tag_sha: impl AsRef<str>) -> reqwest::Result<GitTag> {
        self.get_json(&self.github_repos_api, git_tags_endpoint(tag_sha))
            .await
    }

    /// Sends a `GET` request to the given URL, throws if the status is not
    /// success, parses the response as JSON, and returns as the given type.
    async fn get_json<T: DeserializeOwned>(
        &self,
        data: &ApiData,
        endpoint: impl AsRef<str>,
    ) -> reqwest::Result<T> {
        self.get(data, endpoint).await?.json().await
    }

    /// Sends a `GET` request to the given URL, throws if the status is not
    /// success, and returns the [`Response`].
    async fn get(&self, data: &ApiData, endpoint: impl AsRef<str>) -> reqwest::Result<Response> {
        self.request(Method::GET, data, endpoint)
            .await?
            .error_for_status()
    }

    /// Sends a `HEAD` request to the given URL, and returns the [`Response`].
    async fn head(&self, data: &ApiData, endpoint: impl AsRef<str>) -> reqwest::Result<Response> {
        self.request(Method::HEAD, data, endpoint).await
    }

    /// Creates and returns a [`RequestBuilder`] with the given method,
    /// [`ApiData`] and endpoint.
    async fn request(
        &self,
        method: Method,
        data: &ApiData,
        endpoint: impl AsRef<str>,
    ) -> reqwest::Result<Response> {
        tracing::trace!(
            base_uri = data.base_url,
            endpoint = endpoint.as_ref(),
            "sending request"
        );

        let mut request = self
            .client
            .request(method, format!("{}{}", &data.base_url, endpoint.as_ref()))
            .headers(data.headers.clone())
            .build()?;

        let headers = request.headers_mut();
        if !headers.contains_key(USER_AGENT) {
            headers.insert(USER_AGENT, HeaderValue::from_static(USER_AGENT_VALUE));
        }

        self.client.execute(request).await
    }
}
