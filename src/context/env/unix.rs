use std::path::PathBuf;

use anyhow::anyhow;

use crate::util;

use super::ContextEnv;

const SOURCE_LINE: &str = r#". "{env_sh}""#;

pub struct UnixContextEnv {
    env_sh: PathBuf,
}

impl UnixContextEnv {
    pub fn new<P>(env_sh: P) -> UnixContextEnv
    where
        P: Into<PathBuf>,
    {
        UnixContextEnv {
            env_sh: env_sh.into(),
        }
    }

    fn source_line(&self) -> String {
        SOURCE_LINE.replace("{env_sh}", &format!("{}", self.env_sh.display()))
    }
}

impl ContextEnv for UnixContextEnv {
    fn add(&self) -> anyhow::Result<bool> {
        let source_line = self.source_line();

        let profile = dirs::home_dir()
            .ok_or_else(|| anyhow!("couldn't find home directory"))?
            .join(".profile");

        if util::fs::exists_and_contains_line(&profile, &source_line)? {
            return Ok(false);
        }

        util::fs::write_or_append(&profile, format!("\n{source_line}\n").as_bytes())?;

        Ok(true)
    }

    fn remove(&self) -> anyhow::Result<bool> {
        let source_line = self.source_line();

        let profile = dirs::home_dir()
            .ok_or_else(|| anyhow!("couldn't find home directory"))?
            .join(".profile");

        if !util::fs::exists_and_contains_line(&profile, &source_line)? {
            return Ok(false);
        }

        util::fs::replace(&profile, &source_line, "")?;

        Ok(true)
    }
}
