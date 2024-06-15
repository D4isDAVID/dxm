use std::path::{Path, PathBuf};

const HOME_VAR: &str = "DXM_HOME";

pub struct Home {
    path: PathBuf,
}

impl Home {
    pub fn from_env_or<P>(path: P) -> Self
    where
        P: Into<PathBuf> + Clone,
    {
        if let Some(home) = Self::from_env() {
            return home;
        }

        Self::from_path(path.clone())
    }

    fn from_env() -> Option<Self> {
        if let Ok(var) = std::env::var(HOME_VAR) {
            let path = PathBuf::from(var);
            return Some(Self::from_path(path));
        }

        None
    }

    fn from_path<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = path.into();
        log::debug!("using home path {path:?}");

        Self { path }
    }

    pub fn path(&self) -> &Path {
        self.path.as_ref()
    }

    pub fn bin_dir(&self) -> PathBuf {
        self.path.join("bin")
    }

    pub fn dxm_exe(&self) -> PathBuf {
        self.bin_dir()
            .join("dxm")
            .with_extension(std::env::consts::EXE_EXTENSION)
    }

    pub fn env_sh(&self) -> PathBuf {
        self.path.join("env.sh")
    }
}
