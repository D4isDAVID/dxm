use std::path::PathBuf;

use anyhow::bail;

pub trait PathUtil: Sized {
    fn expect_file(self) -> anyhow::Result<Self>;
    fn expect_dir(self) -> anyhow::Result<Self>;
}

impl PathUtil for PathBuf {
    fn expect_file(self) -> anyhow::Result<Self> {
        if !self.is_file() {
            bail!("this is not a file");
        }

        Ok(self)
    }

    fn expect_dir(self) -> anyhow::Result<Self> {
        if !self.is_dir() {
            bail!("this is not a directory");
        }

        Ok(self)
    }
}
