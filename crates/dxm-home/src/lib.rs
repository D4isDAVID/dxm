//! A crate for installing and updating `dxm`.

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use reqwest::blocking::Client;
use update::UpdatePlatform;

use crate::update::github::Release;

mod env_path;
pub mod update;

/// The default name of the dxm installation directory.
pub const HOME_DIR: &str = ".dxm";
/// The environment variable that may be used to set the dxm installation path.
pub const HOME_ENV: &str = "DXM_HOME";

#[cfg(unix)]
const ENV_SCRIPT: &str = include_str!("env_path/env.sh");

/// Represents a dxm installation.
pub struct Home {
    /// The path of the installation directory.
    path: PathBuf,
}

impl Default for Home {
    fn default() -> Self {
        Self::from_env_or(Self::default_path())
    }
}

impl Home {
    /// Constructs and returns a new `Home` instance.
    pub fn new<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = path.into();

        log::debug!("using home path: {}", path.display());

        Self { path }
    }

    /// Constructs and returns a new `Home` by determining the path using
    /// environment variables.
    pub fn from_env() -> Result<Self, std::env::VarError> {
        let var = std::env::var(HOME_ENV)?;
        let path = PathBuf::from(var);

        Ok(Self::new(path))
    }

    /// Constructs and returns a new `Home` by determining the path using
    /// environment variables, or the given path if the enviroment doesn't
    /// define the path.
    pub fn from_env_or<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::from_env().unwrap_or_else(|_| Self::new(path))
    }

    /// Constructs and returns a new `Home` by determining the path using
    /// environment variables, or the user's home directory if the environment
    /// doesn't define the path.
    pub fn default_path() -> PathBuf {
        dirs::home_dir().map_or_else(|| PathBuf::from(HOME_DIR), |p| p.join(HOME_DIR))
    }

    /// Returns whether the installation directory exists.
    pub fn exists(&self) -> std::io::Result<bool> {
        self.path().try_exists()
    }

    /// Sets up the installation, copying the exe from the given path to the
    /// nested `bin` folder.
    pub fn setup<P>(&self, exe_path: P) -> std::io::Result<()>
    where
        P: AsRef<Path>,
    {
        let exe_path = exe_path.as_ref();

        let bin_dir = self.bin_dir();

        log::trace!("setting up home directories");
        fs_err::create_dir_all(self.path())?;
        fs_err::create_dir_all(&bin_dir)?;

        #[cfg(unix)]
        {
            let bin_dir = format!("{}", bin_dir.display());

            log::trace!("writing env script");
            fs_err::write(self.env_sh(), ENV_SCRIPT.replace("{dxm_bin}", &bin_dir))?;
        }

        if !self.is_current_exe_dxm()? {
            log::trace!("copying executable");
            fs_err::copy(exe_path, self.dxm_exe())?;
        }

        Ok(())
    }

    /// Downloads a new binary and updates the installation.
    pub fn update(
        &self,
        client: &Client,
        release: &Release,
        platform: &UpdatePlatform,
    ) -> Result<(), Box<dyn Error>> {
        let dir = crate::update::download_temp_dir(client, release, platform)?;
        let exe = platform.exe_path(&dir);

        if self.is_current_exe_dxm()? {
            log::trace!("replacing self with updated executable");
            self_replace::self_replace(exe)?;
        } else {
            log::trace!("copying updated executable");
            fs_err::copy(&exe, self.dxm_exe())?;
        }

        Ok(())
    }

    /// Removes the installation.
    pub fn uninstall(&self) -> std::io::Result<()> {
        let home = self.path();

        if self.is_current_exe_dxm()? {
            log::trace!("deleting self");
            self_replace::self_delete_outside_path(home)?;
        }

        log::trace!("deleting home directory");
        fs_err::remove_dir_all(home)?;

        Ok(())
    }

    /// Returns whether the installation path exists in the environment `PATH`.
    pub fn in_env_path(&self) -> std::io::Result<bool> {
        env_path::contains(self.bin_dir(), self.env_sh())
    }

    /// Adds the installation path to the environment `PATH`.
    pub fn add_to_env_path(&self) -> std::io::Result<()> {
        env_path::add(self.bin_dir(), self.env_sh())
    }

    /// Removes the installation path from the environment `PATH`.
    pub fn remove_from_env_path(&self) -> std::io::Result<()> {
        env_path::remove(self.bin_dir(), self.env_sh())
    }

    /// Returns the installation path.
    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    /// Returns the installation's `bin` directory path.
    pub fn bin_dir(&self) -> PathBuf {
        self.path().join("bin")
    }

    /// Returns the installation's `dxm` binary path.
    pub fn dxm_exe(&self) -> PathBuf {
        self.bin_dir()
            .join("dxm")
            .with_extension(std::env::consts::EXE_EXTENSION)
    }

    /// Returns the installation's `env.sh` script path.
    ///
    /// This script is used in linux installations to be ran by `.profile`
    /// files, to add the installation path to the environment `PATH`.
    pub fn env_sh(&self) -> PathBuf {
        self.path().join("env").with_extension("sh")
    }

    /// Returns whether the current binary is the same as the installation's
    /// binary.
    pub fn is_current_exe_dxm(&self) -> std::io::Result<bool> {
        let current_exe = std::env::current_exe()?;
        let dxm_exe = self.dxm_exe();

        Ok(current_exe == dxm_exe)
    }
}
