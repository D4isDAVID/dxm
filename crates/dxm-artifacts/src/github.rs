//! GitHub-specific code for finding information about any FXServer version.

use reqwest::blocking::Client;
use serde::Deserialize;

const GITHUB_TAG_REF_API_URL: &str =
    "https://api.github.com/repos/citizenfx/fivem/git/ref/tags/v1.0.0.{version}";

#[derive(Deserialize)]
struct Ref {
    object: RefObject,
}

#[derive(Deserialize)]
struct RefObject {
    url: String,
}

#[derive(Deserialize)]
struct Tag {
    object: TagObject,
}

#[derive(Deserialize)]
struct TagObject {
    sha: String,
}

/// Fetches and returns the git commit SHA using the given FXServer version.
pub fn get_version_commit_sha<S>(client: &Client, version: S) -> Result<String, reqwest::Error>
where
    S: AsRef<str>,
{
    let version = version.as_ref();

    log::trace!("getting github tag ref");
    let ref_url = GITHUB_TAG_REF_API_URL.replace("{version}", version);
    let github_ref = client
        .get(ref_url)
        .send()?
        .error_for_status()?
        .json::<Ref>()?;

    log::trace!("getting github tag");
    let tag_url = github_ref.object.url;
    let github_tag = client
        .get(tag_url)
        .send()?
        .error_for_status()?
        .json::<Tag>()?;

    Ok(github_tag.object.sha)
}
