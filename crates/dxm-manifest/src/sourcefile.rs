//! Contains functions for sourcefiles used by dxm.
//! Sourcefiles are used for saving the exact version/URL installed for a
//! specific artifact/resource.

use std::path::Path;

pub const SOURCEFILE_NAME: &str = ".dxm-source";

/// Reads the sourcefile in the specified directory.
pub fn read<P>(dir: P) -> std::io::Result<Option<String>>
where
    P: AsRef<Path>,
{
    match fs_err::read_to_string(dir.as_ref().join(SOURCEFILE_NAME)) {
        Ok(v) => Ok(Some(v)),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(None),
            _ => Err(e),
        },
    }
}

/// Writes the sourcefile in the specified directory.
pub fn write<P, S>(dir: P, contents: S) -> std::io::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<[u8]>,
{
    fs_err::write(dir.as_ref().join(SOURCEFILE_NAME), contents)
}
