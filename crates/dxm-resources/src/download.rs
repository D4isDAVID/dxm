use std::{error::Error, fmt::Display, path::Path};

use reqwest::blocking::Client;

mod archive;

pub enum DownloadSource<'a> {
    Archive(&'a Client, String),
}

impl<'a> Display for DownloadSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (url, source) = match self {
            DownloadSource::Archive(_, url) => (url, "archive download"),
        };

        write!(f, "{} ({})", url, source)
    }
}

impl<'a> DownloadSource<'a> {
    pub fn url(&self) -> &str {
        match self {
            DownloadSource::Archive(_, url) => url,
        }
    }

    pub fn download<P, N>(&self, path: P, nested_path: N) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
        N: AsRef<Path>,
    {
        match self {
            DownloadSource::Archive(client, url) => {
                archive::download(client, url, path, nested_path)
            }
        }
    }
}
