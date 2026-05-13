//! A crate for installing third-party resources for FXServer.

use std::{
    error::Error,
    path::{Path, PathBuf},
};

use reqwest::blocking::Client;
use tempfile::TempDir;

use crate::download::DownloadSource;

mod download;
pub mod patch;
mod resolve;

const ROOT_GITIGNORE: &str = "\
*
";

pub use download::format_git_url;

pub fn resolve<'a, S>(
    client: &'a Client,
    download_url: S,
) -> Result<DownloadSource<'a>, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let download_url = download_url.as_ref();

    resolve::download_url(client, download_url)
}

/// Downloads and installs from given source to the given directory path.
/// Returns an updated download URL if the source URL is missing details.
pub fn install<P, N>(
    source: &DownloadSource,
    path: P,
    nested_path: N,
) -> Result<Option<String>, Box<dyn Error>>
where
    P: AsRef<Path>,
    N: AsRef<Path>,
{
    let path = path.as_ref();
    let nested_path = nested_path.as_ref();

    let vacated_dir = VacatedDir::temp(path)?;

    fs_err::create_dir_all(path)?;

    let result = source.download(path, nested_path);

    if result.is_err()
        && let Some(vacated_dir) = vacated_dir
    {
        vacated_dir.bring_back()?;
    }

    let new_url = result?;
    fs_err::write(path.join(".gitignore"), ROOT_GITIGNORE)?;

    Ok(new_url)
}

/// Used to move a directory to a temporary location to be later brought back or
/// deleted.
pub struct VacatedDir {
    vacated_path: PathBuf,
    dest_path: PathBuf,
    #[allow(dead_code)]
    temp_dir: Option<TempDir>,
}

impl VacatedDir {
    pub fn new<A, B>(from: A, to: B) -> Result<Option<Self>, Box<dyn Error>>
    where
        A: AsRef<Path>,
        B: AsRef<Path>,
    {
        let from = from.as_ref();
        let to = to.as_ref();

        Ok(if !is_dir_with_files(from)? {
            None
        } else {
            fs_err::create_dir_all(to)?;

            log::debug!("vacating {} to {}", from.display(), to.display());

            move_dir_contents(from, to)?;

            Some(Self {
                vacated_path: from.into(),
                dest_path: to.into(),
                temp_dir: None,
            })
        })
    }

    pub fn temp<P>(path: P) -> Result<Option<Self>, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        Ok(if !is_dir_with_files(path)? {
            None
        } else {
            let temp_dir = TempDir::with_suffix("dxm-vacated")?;

            log::debug!(
                "vacating {} to temp dir {}",
                path.display(),
                temp_dir.path().display()
            );

            move_dir_contents(path, &temp_dir)?;

            Some(Self {
                vacated_path: path.into(),
                dest_path: temp_dir.path().into(),
                temp_dir: Some(temp_dir),
            })
        })
    }

    pub fn bring_back(self) -> Result<(), Box<dyn Error>> {
        log::debug!(
            "bringing back {} to {}",
            self.vacated_path.display(),
            self.dest_path.display()
        );

        if is_dir_with_files(&self.vacated_path)? {
            // remove directory contents but keep directory itself
            fs_err::remove_dir_all(&self.vacated_path)?;
        }

        fs_err::create_dir_all(&self.vacated_path)?;

        move_dir_contents(self.dest_path, self.vacated_path)?;

        Ok(())
    }
}

fn move_dir_contents<A, B>(from: A, to: B) -> fs_extra::error::Result<u64>
where
    A: AsRef<Path>,
    B: AsRef<Path>,
{
    fs_extra::dir::move_dir(
        from,
        to,
        &fs_extra::dir::CopyOptions::new().content_only(true),
    )
    .inspect_err(|err| {
        if let fs_extra::error::ErrorKind::Io(err) = &err.kind {
            log::debug!("io error moving directory contents: {err}")
        }
    })
}

fn is_dir_with_files<P>(path: P) -> std::io::Result<bool>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    if !path.is_dir() {
        return Ok(false);
    };

    for entry in fs_err::read_dir(path)? {
        if entry?.file_type()?.is_file() {
            return Ok(true);
        }
    }

    Ok(false)
}
