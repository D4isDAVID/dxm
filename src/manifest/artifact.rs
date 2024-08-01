use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::util::{path::PathUtil, result::ResultUtil};

#[cfg(unix)]
const FXSERVER_EXECUTABLE: &str = "run.sh";
#[cfg(windows)]
const FXSERVER_EXECUTABLE: &str = "FXServer.exe";

#[derive(Serialize, Deserialize)]
pub struct Artifact {
    dir: PathBuf,
}

impl Artifact {
    pub fn dir<P>(&self, base_path: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        base_path
            .as_ref()
            .join(&self.dir)
            .canonical_dir()
            .prefix_err("invalid artifact path")
    }

    pub fn fxserver_exe<P>(&self, base_path: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        self.dir(base_path)?
            .join(FXSERVER_EXECUTABLE)
            .canonical_file()
            .prefix_err(format!("invalid {} inside artifact", FXSERVER_EXECUTABLE))
    }
}
