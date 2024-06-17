use std::path::{Path, PathBuf};

use anyhow::anyhow;
use env::{get_cli_context_env, ContextEnv};
use lazycell::LazyCell;
use paths::ContextPaths;

use crate::{home::Home, manifest::Manifest, server::Server};

pub mod env;
pub mod paths;

const DEFAULT_HOME_DIR: &str = ".dxm";
#[cfg(not(windows))]
const ENV_SCRIPT: &str = include_str!("./context/env/env.sh");

pub struct CliContext {
    home: Home,
    paths: ContextPaths,
    env: Box<dyn ContextEnv>,
    manifest: LazyCell<Manifest>,
    server: LazyCell<Server>,
}

impl CliContext {
    pub fn new_default() -> anyhow::Result<CliContext> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| anyhow!("couldn't find home directory"))?
            .join(DEFAULT_HOME_DIR);

        Ok(CliContext::new(&home_dir))
    }

    pub fn new<P>(home_dir: P) -> CliContext
    where
        P: Into<PathBuf> + Clone,
    {
        let home = Home::from_env_or(home_dir);
        let env_sh = home.env_sh();
        let bin_dir = home.bin_dir();

        CliContext {
            home,
            paths: ContextPaths::new(),
            env: get_cli_context_env(env_sh, bin_dir),
            manifest: LazyCell::new(),
            server: LazyCell::new(),
        }
    }

    pub fn home(&self) -> &Home {
        &self.home
    }

    pub fn paths(&self) -> &ContextPaths {
        &self.paths
    }

    pub fn env(&self) -> &dyn ContextEnv {
        &*self.env
    }

    pub fn find_manifest<P>(&mut self, path: Option<P>) -> anyhow::Result<()>
    where
        P: AsRef<Path>,
    {
        let path = path.map_or_else(
            || Manifest::find_file(self.paths.cwd()?),
            |p| Manifest::find_file(p),
        )?;

        log::debug!("using manifest path {}", path.display());
        self.paths.set_manifest(path);

        Ok(())
    }

    pub fn manifest(&self) -> anyhow::Result<&Manifest> {
        self.manifest.try_borrow_with(|| {
            let path = self
                .paths
                .manifest()
                .ok_or_else(|| anyhow!("no manifest path"))?;

            Manifest::from_file(path)
        })
    }

    pub fn server(&self) -> anyhow::Result<&Server> {
        self.server.try_borrow_with(|| {
            let manifest = self.manifest()?;
            std::env::set_current_dir(
                self.paths
                    .manifest()
                    .ok_or_else(|| anyhow!("no manifest path"))?
                    .parent()
                    .ok_or_else(|| anyhow!("no parent for manifest path"))?,
            )?;

            let server = Server::from_manifest(manifest);
            std::env::set_current_dir(self.paths.cwd()?)?;

            server
        })
    }

    pub fn exe_in_home(&self) -> anyhow::Result<bool> {
        let current_exe = self.paths.exe()?;
        let home_exe = self.home.dxm_exe();

        if !home_exe.try_exists()? {
            return Ok(false);
        }

        Ok(current_exe == dunce::canonicalize(home_exe)?)
    }

    pub fn setup_home(&self) -> anyhow::Result<()> {
        let bin_dir = self.home.bin_dir();

        log::trace!("creating dirs");
        #[allow(clippy::needless_borrows_for_generic_args)]
        fs_err::create_dir_all(&bin_dir)?;

        if self.exe_in_home()? {
            log::trace!("executable already in installation");
        } else {
            log::trace!("copying dxm executable");
            fs_err::copy(self.paths.exe()?, self.home.dxm_exe())?;
        }

        #[cfg(not(windows))]
        {
            fs_err::write(
                self.home.env_sh(),
                ENV_SCRIPT.replace("{dxm_bin}", &format!("{}", bin_dir.display())),
            )?;
        }

        Ok(())
    }

    pub fn uninstall(&self) -> anyhow::Result<()> {
        let home_dir = self.home.path();

        log::trace!("deleting self");
        self_replace::self_delete_outside_path(home_dir)?;

        log::trace!("deleting home dir");
        fs_err::remove_dir_all(home_dir)?;

        Ok(())
    }
}
