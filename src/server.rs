use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    manifest::Manifest,
    util::{path::PathUtil, result::ResultUtil},
};

#[cfg(not(windows))]
const FXSERVER_EXECUTABLE: &str = "run.sh";
#[cfg(windows)]
const FXSERVER_EXECUTABLE: &str = "FXServer.exe";

pub struct Server {
    artifact: PathBuf,
    data: PathBuf,
    cfg: PathBuf,
}

impl Server {
    pub fn from_manifest(manifest: &Manifest) -> anyhow::Result<Server> {
        let server = manifest.server();

        Ok(Server {
            artifact: { dunce::canonicalize(server.artifact_dir())?.expect_dir() }
                .prefix_err("invalid artifact path")?,
            data: { dunce::canonicalize(server.data_dir())?.expect_dir() }
                .prefix_err("invalid data path")?,
            cfg: { dunce::canonicalize(server.cfg_file())?.expect_file() }
                .prefix_err("invalid cfg path")?,
        })
    }

    pub fn artifact(&self) -> &Path {
        &self.artifact
    }

    pub fn data(&self) -> &Path {
        &self.data
    }

    pub fn cfg(&self) -> &Path {
        &self.cfg
    }

    pub fn fxserver_exe(&self) -> PathBuf {
        self.artifact.join(FXSERVER_EXECUTABLE)
    }

    pub fn run<V, S>(&self, server_args: V) -> anyhow::Result<()>
    where
        V: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let server_args = server_args.into_iter();

        let exe = self.fxserver_exe();

        log::debug!("running server with {}", exe.display());
        log::debug!("using data path {}", self.data.display());
        log::debug!("using cfg path {}", self.cfg.display());

        Command::new(&exe)
            .current_dir(&self.data)
            .args(vec!["+exec", &self.cfg.display().to_string()])
            .args(server_args)
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn run_tx<P, V, S>(&self, profile: P, server_args: V) -> anyhow::Result<()>
    where
        P: AsRef<str>,
        V: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let profile = profile.as_ref();
        let server_args = server_args.into_iter();

        let exe = self.fxserver_exe();

        log::debug!("running txAdmin with {}", exe.display());
        log::debug!("using profile {profile}");

        Command::new(&exe)
            .args(vec!["+set", "serverProfile", profile])
            .args(server_args)
            .spawn()?
            .wait()?;

        Ok(())
    }
}
