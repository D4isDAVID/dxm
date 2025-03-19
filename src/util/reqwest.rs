use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{self, HeaderMap, HeaderValue},
};

const USER_AGENT: &str = concat!(clap::crate_name!(), "/", clap::crate_version!());

const GITHUB_ACCEPT: &str = "application/vnd.github+json";
const GITHUB_API_VERSION_HEADER: &str = "X-GitHub-Api-Version";
const GITHUB_API_VERSION: &str = "2022-11-28";

pub fn client() -> ClientBuilder {
    Client::builder().default_headers(headers())
}

pub fn github_client() -> ClientBuilder {
    let mut headers = headers();
    headers.insert(header::ACCEPT, HeaderValue::from_static(GITHUB_ACCEPT));
    headers.insert(
        GITHUB_API_VERSION_HEADER,
        HeaderValue::from_static(GITHUB_API_VERSION),
    );

    Client::builder().default_headers(headers)
}

fn headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(header::USER_AGENT, HeaderValue::from_static(USER_AGENT));

    headers
}
