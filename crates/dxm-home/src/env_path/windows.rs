use std::path::Path;

use winreg::{RegKey, enums::HKEY_CURRENT_USER};

const ENV_REGKEY: &str = "Environment";
const ENV_PATH: &str = "PATH";

pub fn contains<P>(path: P) -> std::io::Result<bool>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let (_, env_path) = get_env_and_path()?;

    Ok(env_path.split(';').map(Path::new).any(|p| p == path))
}

pub fn add<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let (env, env_path) = get_env_and_path()?;

    env.set_value(ENV_PATH, &format!("{};{env_path}", path.display()))?;

    Ok(())
}

pub fn remove<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let (env, env_path) = get_env_and_path()?;

    env.set_value(
        ENV_PATH,
        &env_path.replace(&format!("{};", path.display()), ""),
    )?;

    Ok(())
}

fn get_env_and_path() -> std::io::Result<(RegKey, String)> {
    let current_user = RegKey::predef(HKEY_CURRENT_USER);

    let (env, _) = current_user.create_subkey(ENV_REGKEY)?;
    let env_path: String = env.get_value(ENV_PATH)?;

    Ok((env, env_path))
}
