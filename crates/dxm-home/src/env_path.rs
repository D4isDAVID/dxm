//! Contains code for adding paths to the environment `PATH`.

use std::path::Path;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

/// On Linux, returns whether the given env script dir exists in `~/.profile`.
///
/// On Windows, returns whether the given bin dir exists in the environment
/// `PATH`.
pub fn contains<P, S>(_bin_dir: P, _env_sh: S) -> std::io::Result<bool>
where
    P: AsRef<Path>,
    S: AsRef<Path>,
{
    #[cfg(unix)]
    return unix::contains(_env_sh);

    #[cfg(windows)]
    return windows::contains(_bin_dir);
}

/// On Linux, adds the given env script dir exists to `~/.profile`.
///
/// On Windows, adds the given bin dir exists to the environment `PATH`.
pub fn add<P, S>(_bin_dir: P, _env_sh: S) -> std::io::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<Path>,
{
    #[cfg(unix)]
    return unix::add(_env_sh);

    #[cfg(windows)]
    return windows::add(_bin_dir);
}

/// On Linux, removes the given env script dir exists from `~/.profile`.
///
/// On Windows, removes the given bin dir exists from the environment `PATH`.
pub fn remove<P, S>(_bin_dir: P, _env_sh: S) -> std::io::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<Path>,
{
    #[cfg(unix)]
    return unix::remove(_env_sh);

    #[cfg(windows)]
    return windows::remove(_bin_dir);
}
