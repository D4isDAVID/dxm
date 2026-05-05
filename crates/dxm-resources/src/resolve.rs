use std::error::Error;

use reqwest::blocking::Client;

use crate::download::DownloadSource;

mod github;

pub fn download_url<S>(client: &'_ Client, url: S) -> Result<DownloadSource<'_>, Box<dyn Error>>
where
    S: Into<String>,
{
    let url = url.into();

    if let Some(github_url) = github::resolve_download_url(client, &url)? {
        Ok(DownloadSource::Archive(client, github_url))
    } else {
        Ok(DownloadSource::Archive(client, url))
    }
}
