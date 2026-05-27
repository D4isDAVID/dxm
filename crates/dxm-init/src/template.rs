//! Contains interfaces for creating and writing FXServer project templates.

use std::path::{Path, PathBuf};

use tokio::{fs, io};

/// Errors that may occur in [`Template::write`].
#[derive(Debug, thiserror::Error)]
pub enum TemplateError {
    /// A file path in the template went outside the base path.
    #[error("supplied path {0} is not relative to the base path")]
    InvalidPath(PathBuf),
    /// An I/O error.
    #[error("io error: {0}")]
    Io(#[from] io::Error),
}

/// Utility type for returning the given type or a [`TemplateError`].
pub type TemplateResult<T> = Result<T, TemplateError>;

/// A project template containing zero or more files.
#[derive(Debug, Clone)]
pub struct Template {
    files: Vec<(PathBuf, Vec<u8>)>,
}

impl Default for Template {
    fn default() -> Self {
        Self::new()
    }
}

impl Template {
    /// Creates and returns a new [`Template`] with no files.
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    /// Utility function to run [`Self::add_file`] multiple times immediately
    /// after creating an instance.
    pub fn file(mut self, dest_path: impl Into<PathBuf>, content: impl Into<Vec<u8>>) -> Self {
        self.add_file(dest_path, content);

        self
    }

    /// Adds a new file to the template with the given destination path and
    /// content. The given destination path must not go out of the base path
    /// or [`Self::write`] will return an error.
    pub fn add_file(&mut self, dest_path: impl Into<PathBuf>, content: impl Into<Vec<u8>>) {
        self.files.push((dest_path.into(), content.into()));
    }

    /// Writes the template in the given destination directory path.
    pub async fn write(&self, dest_path: impl AsRef<Path>) -> TemplateResult<()> {
        let dest_path = dest_path.as_ref();

        fs::create_dir_all(dest_path).await?;
        let dest_path = fs::canonicalize(dest_path).await?;

        for (file_path, contents) in self.files.iter() {
            let path = dest_path.join(file_path);

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).await?;

                let parent = fs::canonicalize(parent).await?;
                if !parent.starts_with(&dest_path) {
                    return Err(TemplateError::InvalidPath(file_path.clone()));
                }
            }

            fs::write(path, contents).await?;
        }

        Ok(())
    }
}
