use std::{
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use fs_err::OpenOptions;

pub fn replace<P, S>(path: P, from: S, to: &str) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let path = path.as_ref();
    let from = from.as_ref();

    let content = fs_err::read_to_string(path)?;
    let new = content.replace(from, to);

    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;

    Ok(file.write(new.as_bytes())?)
}

pub fn exists_and_contains_line<P, S>(path: P, line: S) -> anyhow::Result<bool>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let path = path.as_ref();
    let line = line.as_ref();

    match OpenOptions::new().read(true).open(path) {
        Ok(file) => Ok(BufReader::new(file).lines().any(|r| match r {
            Ok(l) => l == line,
            Err(_) => false,
        })),
        Err(err) => {
            if err.kind() != std::io::ErrorKind::NotFound {
                anyhow::bail!(err);
            }

            Ok(false)
        }
    }
}

pub fn write_or_append<P>(path: P, buf: &[u8]) -> anyhow::Result<()>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    let mut writer = match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(file) => BufWriter::new(file),
        Err(err) => {
            if err.kind() != std::io::ErrorKind::AlreadyExists {
                anyhow::bail!(err);
            }

            match OpenOptions::new().append(true).open(path) {
                Ok(file) => BufWriter::new(file),
                Err(err) => anyhow::bail!(err),
            }
        }
    };

    writer.write_all(buf)?;
    Ok(())
}
