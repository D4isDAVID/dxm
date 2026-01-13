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

pub fn resolve_download_url<S>(client: &Client, url: S) -> Result<Option<String>, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let url = url.as_ref();

    if let Some((prefix, repo)) = url.trim().split_once("github.com") {
        if !prefix.is_empty() && !prefix.ends_with("//") {
            Err(InvalidGithubUrlError::new(
                InvalidGithubUrlErrorKind::InvalidLink,
            ))?;
        }

        let parts: Vec<&str> = repo.split('/').collect();

        let repo_author = parts
            .get(1)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoAuthor))?;
        let repo_name = parts
            .get(2)
            .filter(|s| !s.is_empty())
            .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoName))?;
        let repo_name = repo_name.strip_suffix(".git").unwrap_or(repo_name);
        let repo = format!("{}/{}", repo_author, repo_name);

        let link_type = parts.get(3).unwrap_or(&"");
        let is_release = link_type == &"releases";
        let link_data = parts.get(4).unwrap_or(&"");
        let release_data = if link_data == &"tag" || link_data == &"download" {
            parts.get(5).unwrap_or(&"")
        } else {
            link_data
        };

        let result = if link_type.is_empty()
            || link_data.is_empty()
            || (is_release && release_data.is_empty())
        {
            api::get_latest_release_archive_url(client, &repo)
                .or_else(|_| api::get_default_branch_archive_url(client, repo))?
        } else if is_release {
            api::get_release_archive_url(client, repo, release_data)?
        } else {
            api::get_branch_or_commit_archive_url(client, repo, link_data)?
        };

        Ok(Some(result))
    } else {
        Ok(None)
    }
}
