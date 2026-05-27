//! Contains interfaces to interact with Cfx.re [Runtime Artifacts storage].
//!
//! [runtime artifacts storage]: https://changelogs-live.fivem.net/api/changelog/versions/win32/server

use std::path::Path;

use async_compression::tokio::bufread::XzDecoder;
use async_zip::{error::ZipError, tokio::read::seek::ZipFileReader};
use tokio::{
    fs,
    io::{self, AsyncBufRead, AsyncSeek, AsyncWriteExt},
};
use tokio_util::compat::FuturesAsyncReadCompatExt;

/// The base URL for Cfx.re's Runtime Artifacts storage.
pub const BASE_URL: &str = "https://runtime.fivem.net/artifacts";

/// The project name for FiveM builds on Cfx.re's Runtime Artifacts storage.
const FIVEM_PROJECT: &str = "fivem";

/// The job name for FXServer Windows builds on Cfx.re's Runtime Artifacts
/// storage.
const WINDOWS_SERVER_JOB: &str = "build_server_windows";

/// The job name for FXServer Linux builds on Cfx.re's Runtime Artifacts
/// storage.
const LINUX_SERVER_JOB: &str = "build_proot_linux";

/// The default branch name for FiveM builds on Cfx.re's Runtime Artifacts
/// storage.
const PRIMARY_BRANCH_NAME: &str = "master";

/// The archive name for FXServer Windows builds on Cfx.re's Runtime
/// Artifacts storage.
const WINDOWS_ARCHIVE: &str = "server.zip";

/// The archive name for FXServer Linux builds on Cfx.re's Runtime
/// Artifacts storage.
const LINUX_ARCHIVE: &str = "fx.tar.xz";

/// Returns the endpoint for downloading the FXServer build associated to the
/// given number for the given platform.
pub fn fxserver_endpoint(
    platform: ArtifactsPlatform,
    number: impl AsRef<str>,
    commit_sha: impl AsRef<str>,
) -> String {
    format!(
        "/{FIVEM_PROJECT}/{}/{PRIMARY_BRANCH_NAME}/{}-{}/{}",
        platform.job_name(),
        number.as_ref(),
        commit_sha.as_ref(),
        platform.archive_name()
    )
}

#[derive(Debug, thiserror::Error)]
#[error(transparent)]
#[non_exhaustive]
pub enum ExtractError {
    Zip(#[from] ZipError),
    Io(#[from] io::Error),
}

/// The platforms for which FXServer can be downloaded.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArtifactsPlatform {
    /// Fetch from Artifacts job [`build_server_windows`](https://runtime.fivem.net/artifacts/fivem/build_server_windows/master).
    Windows,
    /// Fetch from Artifacts job [`build_proot_linux`](https://runtime.fivem.net/artifacts/fivem/build_proot_linux/master).
    Linux,
}

impl Default for ArtifactsPlatform {
    /// Returns the platform which is being compiled for.
    fn default() -> Self {
        #[cfg(windows)]
        return Self::Windows;

        #[cfg(unix)]
        return Self::Linux;
    }
}

impl ArtifactsPlatform {
    /// The slice of all possible values of this enum.
    pub const ALL: [Self; 2] = [Self::Windows, Self::Linux];

    /// Returns an iterator of [`Self::ALL`].
    pub fn iter() -> impl Iterator<Item = Self> {
        Self::ALL.into_iter()
    }

    /// The FXServer build job name for the platform.
    pub fn job_name(&self) -> &'static str {
        match self {
            ArtifactsPlatform::Windows => WINDOWS_SERVER_JOB,
            ArtifactsPlatform::Linux => LINUX_SERVER_JOB,
        }
    }

    /// The FXServer build archive name for the platform.
    pub fn archive_name(&self) -> &'static str {
        match self {
            ArtifactsPlatform::Windows => WINDOWS_ARCHIVE,
            ArtifactsPlatform::Linux => LINUX_ARCHIVE,
        }
    }

    /// Extracts the given FXServer build archive to the given destination path,
    /// according to the platform.
    pub async fn extract(
        &self,
        reader: impl AsyncBufRead + AsyncSeek + Unpin,
        dest_path: impl AsRef<Path>,
    ) -> Result<(), ExtractError> {
        let dest_path = dest_path.as_ref();

        match self {
            ArtifactsPlatform::Windows => {
                tracing::trace!(path = ?dest_path, "extracting artifacts zip");

                let mut zip = ZipFileReader::with_tokio(reader).await?;

                fs::create_dir_all(dest_path).await?;

                for i in 0..zip.file().entries().len() {
                    let reader = zip.reader_with_entry(i).await?;
                    let entry = reader.entry();

                    let filename = entry.filename().as_str()?;
                    let outpath = dest_path.join(filename);

                    if entry.dir()? {
                        fs::create_dir_all(outpath).await?;

                        continue;
                    }

                    if let Some(parent) = outpath.parent() {
                        fs::create_dir_all(parent).await?;
                    }

                    let mut outfile = fs::File::create(&outpath).await?;

                    io::copy(&mut reader.compat(), &mut outfile).await?;
                    outfile.flush().await?;
                }

                Ok(())
            }
            ArtifactsPlatform::Linux => {
                tracing::trace!(path = ?dest_path, "extracting artifacts tarball");

                let xz = XzDecoder::new(reader);

                async_tar::Archive::new(xz).unpack(dest_path).await?;

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_fxserver_endpoint_windows() {
        let platform = ArtifactsPlatform::Windows;
        let endpoint = fxserver_endpoint(platform, "123", "abc");

        assert_eq!(
            endpoint,
            "/fivem/build_server_windows/master/123-abc/server.zip"
        );
    }

    #[test]
    fn should_return_fxserver_endpoint_linux() {
        let platform = ArtifactsPlatform::Linux;
        let endpoint = fxserver_endpoint(platform, "123", "abc");

        assert_eq!(
            endpoint,
            "/fivem/build_proot_linux/master/123-abc/fx.tar.xz"
        );
    }
}
