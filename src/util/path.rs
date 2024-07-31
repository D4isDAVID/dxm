use std::path::PathBuf;

use anyhow::bail;

pub trait PathUtil {
    fn canonical_file(&self) -> anyhow::Result<PathBuf>;
    fn canonical_dir(&self) -> anyhow::Result<PathBuf>;
}

impl PathUtil for PathBuf {
    fn canonical_file(&self) -> anyhow::Result<PathBuf> {
        let path = dunce::canonicalize(self)?;
        if !path.is_file() {
            bail!("this is not a file");
        }

        Ok(path)
    }

    fn canonical_dir(&self) -> anyhow::Result<PathBuf> {
        let path = dunce::canonicalize(self)?;
        if !path.is_dir() {
            bail!("this is not a directory");
        }

        Ok(path)
    }
}
