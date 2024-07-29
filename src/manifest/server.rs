use std::{ffi::OsStr, path::PathBuf, process::Command};

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
    pub fn artifact(&self) -> anyhow::Result<PathBuf> {
        (|| dunce::canonicalize(&self.artifact_dir)?.expect_dir())()
            .prefix_err("invalid artifact path")
    }

    pub fn data(&self) -> anyhow::Result<PathBuf> {
        (|| dunce::canonicalize(&self.data_dir)?.expect_dir())().prefix_err("invalid data path")
    }

    pub fn cfg(&self) -> anyhow::Result<PathBuf> {
        (|| dunce::canonicalize(&self.cfg_file)?.expect_file())().prefix_err("invalid cfg path")
    }

    pub fn fxserver_exe(&self) -> anyhow::Result<PathBuf> {
        (|| dunce::canonicalize(self.artifact_dir.join(FXSERVER_EXECUTABLE))?.expect_file())()
            .prefix_err(format!("invalid {} inside artifact", FXSERVER_EXECUTABLE))
    }

    pub fn run<V, S>(&self, server_args: V) -> anyhow::Result<()>
    where
        V: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let data = self.data()?;
        let cfg = self.cfg()?;
        let exe = self.fxserver_exe()?;

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

    pub fn run_tx<P, V, S>(&self, profile: P, server_args: V) -> anyhow::Result<()>
    where
        P: AsRef<str>,
        V: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let profile = profile.as_ref();
        let server_args = server_args.into_iter();

        let exe = self.fxserver_exe()?;

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
