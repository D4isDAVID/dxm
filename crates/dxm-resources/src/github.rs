use std::{error::Error, fmt::Display};

use reqwest::blocking::Client;

mod api;

#[derive(Debug)]
pub enum InvalidGithubUrlErrorKind {
    InvalidLink,
    NoAuthor,
    NoName,
    DefaultFailed,
    ReleaseFailed,
}

impl Display for InvalidGithubUrlErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            InvalidGithubUrlErrorKind::InvalidLink => "invalid link",
            InvalidGithubUrlErrorKind::NoAuthor => "no repository author",
            InvalidGithubUrlErrorKind::NoName => "no repository name",
            InvalidGithubUrlErrorKind::DefaultFailed => "couldn't fetch default branch or release",
            InvalidGithubUrlErrorKind::ReleaseFailed => "couldn't fetch release",
        };

        write!(f, "{}", str)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct InvalidGithubUrlError {
    kind: InvalidGithubUrlErrorKind,
}

impl InvalidGithubUrlError {
    pub fn new(kind: InvalidGithubUrlErrorKind) -> Self {
        Self { kind }
    }

    pub fn kind(&self) -> &InvalidGithubUrlErrorKind {
        &self.kind
    }
}

impl Display for InvalidGithubUrlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid github url: {}", self.kind())?;

        Ok(())
    }
}

impl Error for InvalidGithubUrlError {}

pub fn resolve_download_url<S>(client: &Client, url: S) -> Result<String, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let url = url.as_ref();

    if let Some((prefix, repo)) = url.trim().split_once("github.com/") {
        if !prefix.is_empty() && !prefix.ends_with("//") {
            Err(InvalidGithubUrlError::new(
                InvalidGithubUrlErrorKind::InvalidLink,
            ))?;
        }

        let parts: Vec<&str> = repo.split('/').collect();

        let repo_author = parts
            .first()
            .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoAuthor))?;
        let repo_name = parts
            .get(1)
            .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoName))?;
        let repo = format!("{}/{}", repo_author, repo_name);

        let link_type = parts.get(2).unwrap_or(&"");
        let is_release = link_type == &"releases";
        let link_data = parts.get(3).unwrap_or(&"");
        let release_data = parts.get(4).unwrap_or(&"");

        if link_type.is_empty() || link_data.is_empty() || (is_release && release_data.is_empty()) {
            if let Some(archive_url) = api::get_latest_release_archive(client, &repo)? {
                return Ok(archive_url);
            }

            if let Some(archive_url) = api::get_default_branch_archive(client, repo)? {
                return Ok(archive_url);
            }

            return Err(InvalidGithubUrlError::new(
                InvalidGithubUrlErrorKind::DefaultFailed,
            ))?;
        } else if is_release {
            let archive_url = api::get_release_archive(client, repo, release_data)?.ok_or(
                InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::ReleaseFailed),
            )?;

            Ok(archive_url)
        } else {
            let archive_url = api::get_branch_or_commit_archive(client, repo, link_data)?;

            Ok(archive_url)
        }
    } else {
        Ok(url.into())
    }
}
