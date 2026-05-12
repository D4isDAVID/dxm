use std::error::Error;

use reqwest::blocking::Client;

use crate::download::DownloadSource;

mod git;
mod github;

pub fn download_url<S>(client: &'_ Client, url: S) -> Result<DownloadSource<'_>, Box<dyn Error>>
where
    S: Into<String>,
{
    let url = url.into();

    if let Some((git_rev, git_url)) = git::resolve_url(&url) {
        Ok(DownloadSource::Git(git_url.into(), git_rev.into()))
    } else if let Some(github_url) = github::resolve_download_url(client, &url)? {
        Ok(DownloadSource::Archive(client, github_url))
    } else {
        Ok(DownloadSource::Archive(client, url))
    }
}
