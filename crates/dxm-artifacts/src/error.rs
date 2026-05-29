use async_zip::error::ZipError;
use reqwest::header::InvalidHeaderValue;
use tokio::io;

/// Utility type for returning the given type or an [`Error`].
pub type Result<T> = std::result::Result<T, Error>;

/// Errors that may occur in [`Artifacts`] methods.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error initializing [`Artifacts`].
    #[error("failed to initialize artifacts client: {0}")]
    Init(#[source] reqwest::Error),
    /// Error setting [`Artifacts`] headers.
    #[error("failed to insert artifacts header: {0}")]
    Header(#[from] InvalidHeaderValue),
    /// Error fetching from [`Artifacts`] methods.
    #[error("failed to fetch jg scripts artifacts db: {0}")]
    Fetch(#[source] reqwest::Error, FetchKind),
    /// Error extracting [`Artifacts`] installation.
    #[error("failed to extract artifacts: {0}")]
    Extract(#[from] ExtractError),
}

/// Errors that may occur in [`Artifacts`] fetches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FetchKind {
    /// Error fetching the JG Scripts Artifacts DB.
    JgArtifactsDb,
    /// Error fetching Cfx.re's FXServer version changelog.
    CfxServerVersions,
    /// Error fetching Cfx.re's Artifacts storage.
    CfxArtifacts,
    /// Error fetching GitHub's Git Reference API.
    GithubGitRef,
    /// Error when fetching GitHub's Git Tags API.
    GithubGitTag,
}

pub trait IntoFetchError<T> {
    fn into_fetch_error(self, kind: FetchKind) -> Result<T>;
}

impl<T> IntoFetchError<T> for reqwest::Result<T> {
    fn into_fetch_error(self, kind: FetchKind) -> Result<T> {
        self.map_err(|e| Error::Fetch(e, kind))
    }
}

/// Errors that may occur in [`Artifacts::extract`].
#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum ExtractError {
    /// Error extracting `.zip` archive.
    Zip(#[from] ZipError),
    /// Error extracting `.tar.xz` archive.
    TarXz(#[from] io::Error),
}

pub trait IntoExtractError<T> {
    fn into_extract_error(self, kind: FetchKind) -> Result<T>;
}

impl<T> IntoFetchError<T> for reqwest::Result<T> {
    fn into_extract_error(self, kind: FetchKind) -> Result<T> {
        self.map_err(|e| Error::Fetch(e, kind))
    }
}
