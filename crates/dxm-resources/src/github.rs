use std::{error::Error, fmt::Display};

use reqwest::blocking::Client;

mod api;

#[derive(Debug)]
pub enum InvalidGithubUrlErrorKind {
    InvalidLink,
    NoAuthor,
    NoName,
}

impl Display for InvalidGithubUrlErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            InvalidGithubUrlErrorKind::InvalidLink => "invalid link",
            InvalidGithubUrlErrorKind::NoAuthor => "no repository author",
            InvalidGithubUrlErrorKind::NoName => "no repository name",
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
        let repo_name = repo_name.strip_suffix(".git").unwrap_or(repo_name);
        let repo = format!("{}/{}", repo_author, repo_name);

        let link_type = parts.get(2).unwrap_or(&"");
        let is_release = link_type == &"releases";
        let link_data = parts.get(3).unwrap_or(&"");
        let release_data = if link_data == &"tag" || link_data == &"download" {
            parts.get(4).unwrap_or(&"")
        } else {
            link_data
        };

        if link_type.is_empty() || link_data.is_empty() || (is_release && release_data.is_empty()) {
            api::get_latest_release_archive(client, &repo)
                .or_else(|_| api::get_default_branch_archive(client, repo))
        } else if is_release {
            api::get_release_archive(client, repo, release_data)
        } else {
            let archive_url = api::get_branch_or_commit_archive(client, repo, link_data)?;

            Ok(archive_url)
        }
    } else {
        Ok(url.into())
    }
}
