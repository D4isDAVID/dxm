use std::{
    io::{Read, Seek},
    path::{Path, PathBuf},
};

use tempfile::NamedTempFile;

/// The supported installation platforms.
#[derive(Clone, Copy)]
pub enum UpdatePlatform {
    Windows,
    Linux,
}

impl Default for UpdatePlatform {
    fn default() -> Self {
        #[cfg(windows)]
        return Self::Windows;
        #[cfg(unix)]
        return Self::Linux;
    }
}

impl UpdatePlatform {
    /// Returns the update archive name for the platform.
    pub fn archive_name<S>(&self, tag_name: S) -> String
    where
        S: AsRef<str>,
    {
        let tag_name = tag_name.as_ref();

        let suffix = match self {
            Self::Windows => "windows-x64.zip",
            Self::Linux => "linux-x64.tar.gz",
        };

        format!("dxm-{tag_name}-{suffix}")
    }

    /// Returns the update binary name for the platform, appended to the given
    /// path.
    pub fn exe_path<P>(&self, base: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        let base = base.as_ref();

        base.join(self.exe_name())
    }

    /// Returns the update binary name for the platform.
    pub fn exe_name(&self) -> &'static str {
        match self {
            Self::Windows => "dxm.exe",
            Self::Linux => "dxm",
        }
    }

    /// Decompresses an archive read from the given reader to the given
    /// directory path.
    pub fn decompress<R, P>(&self, reader: R, dir: P) -> std::io::Result<()>
    where
        R: Read + Seek,
        P: AsRef<Path>,
    {
        let dir = dir.as_ref();

        match self {
            UpdatePlatform::Windows => {
                zip::ZipArchive::new(reader)?.extract(dir)?;
            }
            UpdatePlatform::Linux => {
                let mut file = NamedTempFile::with_prefix("dxm")?;

                let mut decoder = flate2::read::GzDecoder::new(reader);
                std::io::copy(&mut decoder, file.as_file_mut())?;

                tar::Archive::new(file.reopen()?).unpack(dir)?;
            }
        };

        Ok(())
    }
}
