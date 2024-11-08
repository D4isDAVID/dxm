use std::path::PathBuf;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;

pub trait ContextEnv {
    fn add(&self) -> anyhow::Result<bool>;

    fn remove(&self) -> anyhow::Result<bool>;
}

#[allow(unused_variables)]
pub fn get_cli_context_env<P>(env_sh: P, bin_dir: P) -> Box<dyn ContextEnv>
where
    P: Into<PathBuf>,
{
    #[cfg(unix)]
    return Box::new(unix::UnixContextEnv::new(env_sh));

    #[cfg(windows)]
    return Box::new(windows::WindowsContextEnv::new(bin_dir));
}
