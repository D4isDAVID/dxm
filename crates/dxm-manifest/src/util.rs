use std::path::{Path, PathBuf, StripPrefixError};

/// Strips the given base from the given path to create a relative path.
///
/// For example, given a `/test/dir` base and a `/test/dir/nested` path, the
/// function returns `nested`.
pub fn relative_path<B, P>(base: B, path: P) -> Result<PathBuf, StripPrefixError>
where
    B: AsRef<Path>,
    P: AsRef<Path>,
{
    path.as_ref().strip_prefix(base).map(ToOwned::to_owned)
}
