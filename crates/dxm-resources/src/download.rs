use std::{error::Error, fmt::Display, path::Path};

use reqwest::blocking::Client;

mod archive;
mod git;

pub enum DownloadSource<'a> {
    Archive(&'a Client, String),
    Git(String, String),
}

impl<'a> Display for DownloadSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            DownloadSource::Archive(_, url) => format!("{url} (archive download)"),
            DownloadSource::Git(url, rev) => format!("{url} (git clone, rev {rev:?})",),
        };

        write!(f, "{str}")
    }
}

impl<'a> DownloadSource<'a> {
    pub fn url(&self) -> String {
        match self {
            DownloadSource::Archive(_, url) => url.clone(),
            DownloadSource::Git(url, rev) => format_git_url(url, rev),
        }
    }

    pub fn download<P, N>(&self, path: P, nested_path: N) -> Result<Option<String>, Box<dyn Error>>
    where
        P: AsRef<Path>,
        N: AsRef<Path>,
    {
        match self {
            DownloadSource::Archive(client, url) => {
                archive::download(client, url, path, nested_path).map(|_| None)
            }
            DownloadSource::Git(url, rev) => git::clone(url, rev, path, nested_path)
                .map(|o| o.map(|resolved_rev| format_git_url(url, &resolved_rev))),
        }
    }
}

pub fn format_git_url(url: &str, rev: &str) -> String {
    format!("git+{rev}+{url}")
}
