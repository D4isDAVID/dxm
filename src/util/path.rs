use std::path::PathBuf;

pub trait PathUtil {
    fn canonical_file(&self) -> anyhow::Result<PathBuf>;
    fn canonical_dir(&self) -> anyhow::Result<PathBuf>;
}

impl PathUtil for PathBuf {
    fn canonical_file(&self) -> anyhow::Result<PathBuf> {
        let path = dunce::canonicalize(self)?;
        if !path.is_file() {
            anyhow::bail!("this is not a file");
        }

        Ok(path)
    }

    fn canonical_dir(&self) -> anyhow::Result<PathBuf> {
        let path = dunce::canonicalize(self)?;
        if !path.is_dir() {
            anyhow::bail!("this is not a directory");
        }

        Ok(path)
    }
}
