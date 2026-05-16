use std::{error::Error, fmt::Display};

use reqwest::blocking::Client;
use url::Url;

mod api;

#[derive(Debug)]
pub enum InvalidGithubUrlErrorKind {
    NoAuthor,
    NoName,
}

impl Display for InvalidGithubUrlErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
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
    let url = Url::parse(url.as_ref())?;

    if url.host_str() != Some("github.com") {
        return Ok(None);
    }

    let parts: Vec<&str> = url
        .path_segments()
        .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoAuthor))?
        .collect();

    let link_type = parts.get(2).cloned().unwrap_or("");

    if link_type == "archive" {
        return Ok(Some(url.into()));
    }

    let repo_author = parts
        .first()
        .cloned()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoAuthor))?;

    let repo_name = parts
        .get(1)
        .cloned()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| InvalidGithubUrlError::new(InvalidGithubUrlErrorKind::NoName))?;

    let repo_name = if link_type.is_empty() {
        repo_name.strip_suffix(".git").unwrap_or(repo_name)
    } else {
        repo_name
    };

    let repo = format!("{}/{}", repo_author, repo_name);
    let is_release = link_type == "releases";

    let link_data = parts.get(3).cloned().unwrap_or("");

    let head = if is_release && (link_data == "tag" || link_data == "download") {
        parts.get(4..).map(|v| v.join("/")).unwrap_or("".to_owned())
    } else {
        parts.get(3..).map(|v| v.join("/")).unwrap_or("".to_owned())
    };

    let result = if link_type.is_empty() || head.is_empty() || (is_release && head.is_empty()) {
        api::get_latest_release_archive_url(client, &repo)
            .or_else(|_| api::get_default_branch_archive_url(client, repo))?
    } else if is_release {
        api::get_release_archive_url(client, repo, head)?
    } else {
        api::get_branch_or_commit_archive_url(client, repo, head)?
    };

    Ok(Some(result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic = "RelativeUrlWithoutBase"]
    fn resolve_returns_error_for_invalid_link() {
        resolve_download_url(&Client::new(), "/github.com/").unwrap();
    }

    #[test]
    #[should_panic = "NoAuthor"]
    fn resolve_returns_error_for_no_author() {
        resolve_download_url(&Client::new(), "https://github.com").unwrap();
    }

    #[test]
    #[should_panic = "NoAuthor"]
    fn resolve_returns_error_for_empty_author() {
        resolve_download_url(&Client::new(), "https://github.com/").unwrap();
    }

    #[test]
    #[should_panic = "NoName"]
    fn resolve_returns_error_for_no_repository() {
        resolve_download_url(&Client::new(), "https://github.com/example").unwrap();
    }

    #[test]
    #[should_panic = "NoName"]
    fn resolve_returns_error_for_empty_repository() {
        resolve_download_url(&Client::new(), "https://github.com/example/").unwrap();
    }
}
