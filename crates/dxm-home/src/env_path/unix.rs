use std::{
    io::Write,
    path::{Path, PathBuf},
};

use fs_err::OpenOptions;

const SOURCE_LINE: &str = r#". "{env_sh}""#;

pub fn contains<P>(path: P) -> std::io::Result<bool>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let (profile, source_line) = get_profile_and_source_line(path)?;

    if !profile.try_exists()? {
        return Ok(false);
    }

    let contents = fs_err::read_to_string(&profile)?;
    let contains = contents.contains(&source_line);

    Ok(contains)
}

pub fn add<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let (profile, source_line) = get_profile_and_source_line(path)?;

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&profile)?;
    file.write_all(format!("\n{source_line}\n").as_bytes())?;

    Ok(())
}

pub fn remove<P>(path: P) -> std::io::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let (profile, source_line) = get_profile_and_source_line(path)?;

    let contents = fs_err::read_to_string(path)?;
    fs_err::write(&profile, contents.replace(&source_line, ""))?;

    Ok(())
}

fn get_profile_and_source_line(env_sh: &Path) -> std::io::Result<(PathBuf, String)> {
    let home = dirs::home_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::NotFound, "couldn't find home directory")
    })?;
    let profile = home.join(".profile");

    let env_sh = format!("{}", env_sh.display());
    let source_line = SOURCE_LINE.replace("{env_sh}", &env_sh);

    Ok((profile, source_line))
}
