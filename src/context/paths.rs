use std::path::{Path, PathBuf};

use lazycell::LazyCell;

pub struct ContextPaths {
    cwd: LazyCell<PathBuf>,
    exe: LazyCell<PathBuf>,
}

impl Default for ContextPaths {
    fn default() -> Self {
        Self {
            cwd: LazyCell::new(),
            exe: LazyCell::new(),
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
                let path = fs_err::canonicalize(std::env::current_dir()?)?;
                Ok(path)
            })
            .map(AsRef::as_ref)
    }

    pub fn exe(&self) -> anyhow::Result<&Path> {
        self.exe
            .try_borrow_with(|| {
                let path = fs_err::canonicalize(std::env::current_exe()?)?;
                Ok(path)
            })
            .map(AsRef::as_ref)
    }
}
