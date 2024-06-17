use std::path::{Path, PathBuf};

use lazycell::LazyCell;

pub struct ContextPaths {
    cwd: LazyCell<PathBuf>,
    exe: LazyCell<PathBuf>,
    manifest: Option<PathBuf>,
}

impl Default for ContextPaths {
    fn default() -> Self {
        Self {
            cwd: LazyCell::new(),
            exe: LazyCell::new(),
            manifest: None,
        }
    }
}

impl ContextPaths {
    pub fn new() -> ContextPaths {
        ContextPaths::default()
    }

    pub fn cwd(&self) -> anyhow::Result<&Path> {
        self.cwd
            .try_borrow_with(|| {
                let path = dunce::canonicalize(std::env::current_dir()?)?;
                Ok(path)
            })
            .map(AsRef::as_ref)
    }

    pub fn exe(&self) -> anyhow::Result<&Path> {
        self.exe
            .try_borrow_with(|| {
                let path = dunce::canonicalize(std::env::current_exe()?)?;
                Ok(path)
            })
            .map(AsRef::as_ref)
    }

    pub fn manifest(&self) -> Option<&PathBuf> {
        self.manifest.as_ref()
    }

    pub fn set_manifest<P>(&mut self, path: P)
    where
        P: Into<PathBuf>,
    {
        self.manifest = Some(path.into());
    }
}
