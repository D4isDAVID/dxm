use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ServerManifest {
    artifact_dir: String,
    data_dir: String,
    cfg_file: String,
}

impl ServerManifest {
    pub fn artifact_dir(&self) -> PathBuf {
        PathBuf::from(&self.artifact_dir)
    }

    pub fn data_dir(&self) -> PathBuf {
        PathBuf::from(&self.data_dir)
    }

    pub fn cfg_file(&self) -> PathBuf {
        PathBuf::from(&self.cfg_file)
    }
}
