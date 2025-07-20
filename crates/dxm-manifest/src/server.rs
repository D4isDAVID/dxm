//! Contains structures that represent server data.

use std::{
    path::{Path, PathBuf, StripPrefixError},
    sync::LazyLock,
};

use serde::{Deserialize, Serialize};

use crate::util::relative_path;

static DEFAULT_DATA: LazyLock<PathBuf> = LazyLock::new(|| PathBuf::from("data"));

/// Represents dxm-managed server data.
#[derive(Serialize, Deserialize)]
pub struct Server {
    /// The server data path.
    ///
    /// Default: `data`
    data: Option<PathBuf>,
}

impl Default for Server {
    fn default() -> Self {
        Self {
            data: Some(DEFAULT_DATA.to_path_buf()),
        }
    }
}

impl Server {
    /// Sets the server data's path relative to the given manifest file path.
    pub fn set_data<M, P>(&mut self, manifest_path: M, data: P) -> Result<(), StripPrefixError>
    where
        M: AsRef<Path>,
        P: AsRef<Path>,
    {
        self.data = Some(relative_path(manifest_path, data)?);

        Ok(())
    }

    /// Returns the server data's path relative to the given manifest file path.
    fn relative_data(&self) -> &PathBuf {
        self.data.as_ref().unwrap_or(&*DEFAULT_DATA)
    }

    /// Returns the server data's path appended to the given manifest file path.
    pub fn data<P>(&self, manifest_path: P) -> PathBuf
    where
        P: AsRef<Path>,
    {
        manifest_path.as_ref().join(self.relative_data())
    }

    /// Returns the server data's path appended to the given manifest file path,
    /// and ensures that it exists by creating it if it doesn't.
    pub fn ensure_data<P>(&self, manifest_path: P) -> std::io::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let path = self.data(manifest_path);

        fs_err::create_dir_all(&path)?;

        Ok(path)
    }

    /// Fills out information about the server inside the given TOML document.
    pub fn fill_toml_item(&self, document: &mut toml_edit::Item) {
        document["data"] = toml_edit::value(self.relative_data().to_string_lossy().into_owned());
    }
}
