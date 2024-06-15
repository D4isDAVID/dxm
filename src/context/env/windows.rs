use std::path::{Path, PathBuf};

use winreg::{enums::HKEY_CURRENT_USER, RegKey};

use super::ContextEnv;

const ENV_REGKEY: &str = "Environment";
const ENV_PATH: &str = "PATH";

pub struct WindowsContextEnv {
    bin_dir: PathBuf,
}

impl WindowsContextEnv {
    pub fn new<P>(bin_dir: P) -> WindowsContextEnv
    where
        P: Into<PathBuf>,
    {
        WindowsContextEnv {
            bin_dir: bin_dir.into(),
        }
    }
}

impl ContextEnv for WindowsContextEnv {
    fn add(&self) -> anyhow::Result<bool> {
        let path = fs_err::canonicalize(&self.bin_dir)?;
        let env = env_regkey()?;
        let env_path: String = env.get_value(ENV_PATH)?;

        if env_path_includes(&env_path, &path) {
            return Ok(false);
        }

        env.set_value(ENV_PATH, &format!("{};{env_path}", path.display()))?;

        Ok(true)
    }

    fn remove(&self) -> anyhow::Result<bool> {
        let path = fs_err::canonicalize(&self.bin_dir)?;
        let env = env_regkey()?;
        let env_path: String = env.get_value(ENV_PATH)?;

        if !env_path_includes(&env_path, &path) {
            return Ok(false);
        }

        env.set_value(
            ENV_PATH,
            &env_path.replace(&format!("{};", path.display()), ""),
        )?;

        Ok(true)
    }
}

fn env_path_includes(env_path: &str, path: &Path) -> bool {
    env_path
        .split(';')
        .map(|s| fs_err::canonicalize(Path::new(s)))
        .any(|r| match r {
            Ok(p) => p == path,
            Err(_) => false,
        })
}

fn env_regkey() -> anyhow::Result<RegKey> {
    let current_user = RegKey::predef(HKEY_CURRENT_USER);
    let (env, _) = current_user.create_subkey(ENV_REGKEY)?;

    Ok(env)
}
