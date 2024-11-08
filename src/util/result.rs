use std::fmt::Display;

pub trait ResultUtil<T> {
    fn prefix_err<S>(self, prefix: S) -> Result<T, anyhow::Error>
    where
        S: AsRef<str> + Display;
}

impl<T, E: Display> ResultUtil<T> for Result<T, E> {
    fn prefix_err<S>(self, prefix: S) -> Result<T, anyhow::Error>
    where
        S: AsRef<str> + Display,
    {
        self.map_err(|e| anyhow::anyhow!("{}: {}", prefix, e))
    }
}
