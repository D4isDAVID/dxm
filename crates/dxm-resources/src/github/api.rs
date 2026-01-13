use std::error::Error;

use reqwest::blocking::Client;
use serde::Deserialize;

const GITHUB_REPO_ARCHIVE_URL: &str = "https://github.com/{repo}/archive/{commit}.zip";
const GITHUB_REPOS_API_URL: &str = "https://api.github.com/repos/{repo}";

#[derive(Deserialize)]
pub struct GithubRelease {
    tag_name: String,
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize)]
pub struct GithubReleaseAsset {
    browser_download_url: String,
}

#[derive(Deserialize)]
pub struct GithubRepository {
    default_branch: String,
}

pub fn get_latest_release_archive_url<S>(client: &Client, repo: S) -> Result<String, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let repo = repo.as_ref();

    let release_url = repo_release_api_url(repo, "latest");

    get_release_archive_url_internal(client, repo, release_url)
}

pub fn get_release_archive_url<R, S>(
    client: &Client,
    repo: R,
    tag: S,
) -> Result<String, Box<dyn Error>>
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let repo = repo.as_ref();
    let tag = tag.as_ref();

    let release_url = repo_release_api_url(repo, format!("tags/{}", tag));

    get_release_archive_url_internal(client, repo, release_url)
}

pub fn get_default_branch_archive_url<S>(client: &Client, repo: S) -> Result<String, Box<dyn Error>>
where
    S: AsRef<str>,
{
    let repo = repo.as_ref();

    let repo_url = repo_api_url(repo);
    let repo = client
        .get(repo_url)
        .send()?
        .error_for_status()?
        .json::<GithubRepository>()?;

    Ok(repo.default_branch)
}

pub fn get_branch_or_commit_archive_url<R, S>(
    client: &Client,
    repo: R,
    commit: S,
) -> Result<String, Box<dyn Error>>
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let repo = repo.as_ref();
    let commit = commit.as_ref();

    let branch_url = repo_branch_api_url(repo, commit);
    let response = client.head(branch_url).send()?;

    if response.status().is_success() {
        return Ok(get_branch_archive_url(repo, commit));
    }

    Ok(get_commit_archive_url(repo, commit))
}

fn get_branch_archive_url<R, S>(repo: R, branch: S) -> String
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let repo = repo.as_ref();
    let branch = branch.as_ref();

    repo_archive_url(repo, format!("refs/heads/{}", branch))
}

fn get_commit_archive_url<R, S>(repo: R, commit: S) -> String
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let repo = repo.as_ref();
    let commit = commit.as_ref();

    repo_archive_url(repo, commit)
}

fn get_release_archive_url_internal<R, S>(
    client: &Client,
    repo: R,
    release_url: S,
) -> Result<String, Box<dyn Error>>
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let release = client
        .get(release_url.as_ref())
        .send()?
        .error_for_status()?
        .json::<GithubRelease>()?;

    let archive_url = release
        .assets
        .first()
        .map(|a| a.browser_download_url.clone())
        .unwrap_or_else(|| get_tag_archive_url(repo, release.tag_name));

    Ok(archive_url)
}

fn get_tag_archive_url<R, S>(repo: R, tag: S) -> String
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let repo = repo.as_ref();
    let tag = tag.as_ref();

    repo_archive_url(repo, format!("refs/tags/{}", tag))
}

fn repo_archive_url<R, S>(repo: R, commit: S) -> String
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let repo = repo.as_ref();
    let commit = commit.as_ref();

    GITHUB_REPO_ARCHIVE_URL
        .replace("{repo}", repo)
        .replace("{commit}", commit)
}

fn repo_branch_api_url<R, S>(repo: R, branch: S) -> String
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let branch = branch.as_ref();
    let repo_url = repo_api_url(repo);

    format!("{}/git/refs/heads/{}", repo_url, branch)
}

fn repo_release_api_url<R, S>(repo: R, release: S) -> String
where
    R: AsRef<str>,
    S: AsRef<str>,
{
    let release = release.as_ref();
    let repo_url = repo_api_url(repo);

    format!("{}/releases/{}", repo_url, release)
}

fn repo_api_url<S>(repo: S) -> String
where
    S: AsRef<str>,
{
    let repo = repo.as_ref();

    GITHUB_REPOS_API_URL.replace("{repo}", repo)
}
