use std::path::Path;

use serde::{de::DeserializeOwned, Serialize};

pub fn from_file<P, T>(path: P) -> anyhow::Result<T>
where
    P: AsRef<Path>,
    T: DeserializeOwned,
{
    let path = path.as_ref();
    let contents = fs_err::read_to_string(path)?;

    Ok(toml::from_str(&contents)?)
}

pub fn to_file<P, T>(path: P, value: &T) -> anyhow::Result<()>
where
    P: AsRef<Path>,
    T: Serialize + ?Sized,
{
    let path = path.as_ref();
    let contents = toml::to_string_pretty(value)?;

    fs_err::write(path, contents)?;

    Ok(())
}
