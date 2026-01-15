use std::{
    io::{Read, Seek},
    path::Path,
};

use tempfile::NamedTempFile;
use zip::ZipArchive;

const CFX_ARTIFACTS_FILE_API_URL: &str =
    "https://runtime.fivem.net/artifacts/fivem/{platform}/master/{version}-{commit}/{archive}";

/// The supported FXServer platforms.
#[derive(Clone, Copy)]
pub enum ArtifactsPlatform {
    Windows,
    Linux,
}

impl Default for ArtifactsPlatform {
    fn default() -> Self {
        #[cfg(windows)]
        return Self::Windows;

        #[cfg(unix)]
        return Self::Linux;
    }
}

impl ArtifactsPlatform {
    /// Returns the name in the version changelog URL of the platform.
    pub fn changelogs_name(&self) -> &str {
        match self {
            Self::Windows => "win32",
            Self::Linux => "linux",
        }
    }

    /// Returns the name in the installation archive URL of the platform.
    pub fn runtime_name(&self) -> &str {
        match self {
            Self::Windows => "build_server_windows",
            Self::Linux => "build_proot_linux",
        }
    }

    /// Returns the installation archive name of the platform.
    pub fn archive_name(&self) -> &str {
        match self {
            Self::Windows => "server.zip",
            Self::Linux => "fx.tar.xz",
        }
    }

    /// Returns the installation binary name of the platform.
    pub fn exe_name(&self) -> &str {
        match self {
            Self::Windows => "FXServer.exe",
            Self::Linux => "run.sh",
        }
    }

    /// Returns the installation archive URL of the platform.
    pub fn runtime_url<S, C>(&self, version: S, commit_sha: C) -> String
    where
        S: AsRef<str>,
        C: AsRef<str>,
    {
        let version = version.as_ref();
        let commit_sha = commit_sha.as_ref();

        CFX_ARTIFACTS_FILE_API_URL
            .replace("{platform}", self.runtime_name())
            .replace("{version}", version)
            .replace("{commit}", commit_sha)
            .replace("{archive}", self.archive_name())
    }

    /// Reads an archive from the given reader and decompresses it to the given
    /// directory path.
    pub fn decompress<R, P>(&self, reader: R, dir: P) -> std::io::Result<()>
    where
        R: Read + Seek,
        P: AsRef<Path>,
    {
        match self {
            ArtifactsPlatform::Windows => {
                ZipArchive::new(reader)?.extract(dir)?;
            }
            ArtifactsPlatform::Linux => {
                let mut file = NamedTempFile::with_prefix("dxmfx")?;

                let mut decoder = xz2::read::XzDecoder::new(reader);
                std::io::copy(&mut decoder, file.as_file_mut())?;

                tar::Archive::new(file.reopen()?).unpack(dir)?;
            }
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_runtime_url() {
        assert_eq!(
            ArtifactsPlatform::Windows.runtime_url("1234", "abcd"),
            "https://runtime.fivem.net/artifacts/fivem/build_server_windows/master/1234-abcd/server.zip"
        );
    }
}
