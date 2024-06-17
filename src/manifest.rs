use std::path::{Path, PathBuf};

use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use server::ServerManifest;

use crate::util;

pub mod server;

const MANIFEST_NAME: &str = "dxm.toml";

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    server: ServerManifest,
}

impl Manifest {
    pub fn server(&self) -> &ServerManifest {
        &self.server
    }

    pub fn find_file<P>(dir: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        let mut dir = dir.as_ref();
        let mut path = dir.join(MANIFEST_NAME);

        while !path.try_exists()? {
            dir = dir
                .parent()
                .ok_or_else(|| anyhow!("couldn't find manifest"))?;
            path = dir.join(MANIFEST_NAME);
        }

        Ok(path)
    }

    pub fn from_file<P>(file: P) -> anyhow::Result<Manifest>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();

        util::toml::from_file(file)
    }

    pub fn to_file<P>(&self, file: P) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();

        util::toml::to_file(file, &self)
    }
}
