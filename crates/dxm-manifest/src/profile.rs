//! Contains structures that represent FXServer start profile data.

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub const TXHOST_DATA_PATH: &str = "TXHOST_DATA_PATH";

/// Represents a profile containing data to start FXServer with.
#[derive(Default, Serialize, Deserialize)]
pub struct Profile {
    /// The startup arguments to pass to the FXServer process.
    #[serde(default)]
    server_args: Vec<String>,

    /// The environment variables to set for the FXServer process.
    #[serde(default)]
    env_vars: HashMap<String, String>,
}

impl Profile {
    /// Returns the startup arguments to pass to the FXServer process.
    pub fn server_args(&self) -> Vec<&String> {
        self.server_args.iter().collect()
    }

    /// Returns the environment variables to set for the FXServer process.
    pub fn env_vars(&self) -> HashMap<&String, &String> {
        self.env_vars.iter().collect()
    }

    pub fn set_txhost_data_path<S>(&mut self, txhost_data_path: S)
    where
        S: Into<String>,
    {
        self.env_vars
            .insert(TXHOST_DATA_PATH.to_owned(), txhost_data_path.into());
    }

    pub fn txhost_data_path<P>(&self, manifest_path: P) -> std::io::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let path = manifest_path.as_ref().join(
            self.env_vars
                .get(TXHOST_DATA_PATH)
                .map(|s| s.as_str())
                .unwrap_or("./txData"),
        );

        fs_err::create_dir_all(&path)?;

        Ok(path)
    }
}
