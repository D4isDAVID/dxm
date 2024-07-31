use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};

use crate::util::{path::PathUtil, result::ResultUtil};

#[cfg(not(windows))]
const FXSERVER_EXECUTABLE: &str = "run.sh";
#[cfg(windows)]
const FXSERVER_EXECUTABLE: &str = "FXServer.exe";

#[derive(Serialize, Deserialize)]
pub struct Server {
    artifact_dir: PathBuf,
    data_dir: PathBuf,
    cfg_file: PathBuf,
}

impl Server {
    pub fn artifact<P>(&self, base_path: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        base_path
            .as_ref()
            .join(&self.artifact_dir)
            .canonical_dir()
            .prefix_err("invalid artifact path")
    }

    pub fn data<P>(&self, base_path: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        base_path
            .as_ref()
            .join(&self.data_dir)
            .canonical_dir()
            .prefix_err("invalid data path")
    }

    pub fn cfg<P>(&self, base_path: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        base_path
            .as_ref()
            .join(&self.cfg_file)
            .canonical_file()
            .prefix_err("invalid cfg path")
    }

    pub fn fxserver_exe<P>(&self, base_path: P) -> anyhow::Result<PathBuf>
    where
        P: AsRef<Path>,
    {
        self.artifact(base_path)?
            .join(FXSERVER_EXECUTABLE)
            .canonical_file()
            .prefix_err(format!("invalid {} inside artifact", FXSERVER_EXECUTABLE))
    }

    pub fn run<P, V, S>(&self, base_path: P, server_args: V) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
        V: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let data = self.data(&base_path)?;
        let cfg = self.cfg(&base_path)?;
        let exe = self.fxserver_exe(&base_path)?;

        log::debug!("running server with {}", exe.display());
        log::debug!("using data path {}", data.display());
        log::debug!("using cfg path {}", cfg.display());

        Command::new(&exe)
            .current_dir(data)
            .args(server_args)
            .args(vec!["+exec", &cfg.display().to_string()])
            .spawn()?
            .wait()?;

        Ok(())
    }

    pub fn run_tx<P, S, V, A>(&self, base_path: P, profile: S, server_args: V) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
        S: AsRef<str>,
        V: IntoIterator<Item = A>,
        A: AsRef<OsStr>,
    {
        let profile = profile.as_ref();
        let server_args = server_args.into_iter();

        let exe = self.fxserver_exe(base_path)?;

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
