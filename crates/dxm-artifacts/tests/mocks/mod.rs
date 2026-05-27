use std::path::Path;

use async_compression::tokio::write::XzEncoder;
use async_tempfile::TempFile;
use async_zip::{Compression, ZipEntryBuilder, tokio::write::ZipFileWriter};
use dxm_artifacts::Artifacts;
use reqwest::StatusCode;
use serde_json::json;
use tokio::{fs, io::AsyncWriteExt};
use tokio_util::compat::TokioAsyncReadCompatExt;
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[allow(dead_code)]
pub trait MockArtifacts {
    fn mock(server: &MockServer) -> Self;
}

impl MockArtifacts for Artifacts {
    fn mock(server: &MockServer) -> Self {
        Self::with_urls(
            reqwest::Client::new(),
            server.jg_artifacts_db_uri(),
            server.cfx_changelogs_api_uri(),
            server.cfx_artifacts_uri(),
            server.github_repos_uri(),
        )
    }
}

#[allow(dead_code, async_fn_in_trait)]
pub trait ApiMockServer {
    fn jg_artifacts_db_uri(&self) -> String;
    fn cfx_changelogs_api_uri(&self) -> String;
    fn cfx_artifacts_uri(&self) -> String;
    fn github_repos_uri(&self) -> String;

    async fn with_jg_artifacts_db(self) -> Self;
    async fn with_cfx_changelogs_api(self) -> Self;
    async fn with_cfx_artifacts_windows<B>(self, build_number: &str, bytes: B) -> Self
    where
        B: TryInto<Vec<u8>>,
        <B as TryInto<Vec<u8>>>::Error: std::fmt::Debug;
    async fn with_cfx_artifacts_linux<B>(self, build_number: &str, bytes: B) -> Self
    where
        B: TryInto<Vec<u8>>,
        <B as TryInto<Vec<u8>>>::Error: std::fmt::Debug;
    async fn with_cfx_artifacts_check_windows(self, build_number: &str) -> Self;
    async fn with_cfx_artifacts_check_linux(self, build_number: &str) -> Self;
    async fn with_github_git_ref(self, build_number: &str) -> Self;
    async fn with_github_git_tag(self) -> Self;
}

impl ApiMockServer for MockServer {
    fn jg_artifacts_db_uri(&self) -> String {
        format!("{}/jg", self.uri())
    }

    fn cfx_changelogs_api_uri(&self) -> String {
        format!("{}/changelogs", self.uri())
    }

    fn cfx_artifacts_uri(&self) -> String {
        format!("{}/artifacts", self.uri())
    }

    fn github_repos_uri(&self) -> String {
        format!("{}/repos", self.uri())
    }

    async fn with_jg_artifacts_db(self) -> Self {
        Mock::given(method("GET"))
            .and(path("/jg/json"))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(
                json!({ "recommendedArtifact": "2", "brokenArtifacts": { "3": "test", "3-4": "second test" } }),
            ))
            .mount(&self)
            .await;

        self
    }

    async fn with_cfx_changelogs_api(self) -> Self {
        Mock::given(method("GET"))
            .and(path("/changelogs/changelog/versions/win32/server"))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_json(
                json!({ "latest": "4", "recommended": "1", "support_policy": { "4": "9999-12-31T23:59:59.9999999", "3": "2025-01-01T00:00:00Z", "2": "3000-01-01T00:00:00Z", "1": "9999-12-31T23:59:59.9999999" } }),
            ))
            .mount(&self)
            .await;

        self
    }

    async fn with_cfx_artifacts_windows<B>(self, build_number: &str, bytes: B) -> Self
    where
        B: TryInto<Vec<u8>>,
        <B as TryInto<Vec<u8>>>::Error: std::fmt::Debug,
    {
        Mock::given(method("GET"))
            .and(path(format!(
                "/artifacts/fivem/build_server_windows/master/{build_number}-xyz/server.zip",
            )))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_bytes(bytes))
            .mount(&self)
            .await;

        self.with_cfx_artifacts_check_windows(build_number).await
    }

    async fn with_cfx_artifacts_linux<B>(self, build_number: &str, bytes: B) -> Self
    where
        B: TryInto<Vec<u8>>,
        <B as TryInto<Vec<u8>>>::Error: std::fmt::Debug,
    {
        Mock::given(method("GET"))
            .and(path(format!(
                "/artifacts/fivem/build_proot_linux/master/{build_number}-xyz/fx.tar.xz",
            )))
            .respond_with(ResponseTemplate::new(StatusCode::OK).set_body_bytes(bytes))
            .mount(&self)
            .await;

        self.with_cfx_artifacts_check_linux(build_number).await
    }

    async fn with_cfx_artifacts_check_windows(self, build_number: &str) -> Self {
        Mock::given(method("HEAD"))
            .and(path(format!(
                "/artifacts/fivem/build_server_windows/master/{build_number}-xyz/server.zip",
            )))
            .respond_with(ResponseTemplate::new(StatusCode::OK))
            .mount(&self)
            .await;

        self
    }

    async fn with_cfx_artifacts_check_linux(self, build_number: &str) -> Self {
        Mock::given(method("HEAD"))
            .and(path(format!(
                "/artifacts/fivem/build_proot_linux/master/{build_number}-xyz/fx.tar.xz",
            )))
            .respond_with(ResponseTemplate::new(StatusCode::OK))
            .mount(&self)
            .await;

        self
    }

    async fn with_github_git_ref(self, build_number: &str) -> Self {
        Mock::given(method("GET"))
            .and(path(format!(
                "/repos/citizenfx/fivem/git/ref/tags/v1.0.0.{build_number}"
            )))
            .respond_with(
                ResponseTemplate::new(StatusCode::OK)
                    .set_body_json(json!({ "object": { "sha": "abc" } })),
            )
            .mount(&self)
            .await;

        self
    }

    async fn with_github_git_tag(self) -> Self {
        Mock::given(method("GET"))
            .and(path("/repos/citizenfx/fivem/git/tags/abc"))
            .respond_with(
                ResponseTemplate::new(StatusCode::OK)
                    .set_body_json(json!({ "object": { "sha": "xyz" } })),
            )
            .mount(&self)
            .await;

        self
    }
}

pub async fn assert_extract_contents(path: &Path) {
    let contents = fs::read_to_string(path.join("test.txt")).await.unwrap();

    assert_eq!(contents, "test");
}

pub async fn setup_windows_archive() -> TempFile {
    let file = TempFile::new().await.unwrap();

    let mut writer = ZipFileWriter::new(file.open_rw().await.unwrap().compat());
    let builder = ZipEntryBuilder::new("test.txt".into(), Compression::Deflate);

    writer.write_entry_whole(builder, b"test").await.unwrap();
    writer.close().await.unwrap();

    file
}

pub async fn setup_linux_archive() -> TempFile {
    let file = TempFile::new().await.unwrap();

    let xz = XzEncoder::new(file.open_rw().await.unwrap());
    let mut builder = async_tar::Builder::new(xz);
    let test_data = "test".as_bytes();

    let mut header = async_tar::Header::new_gnu();
    header.set_size(test_data.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    builder
        .append_data(&mut header, "test.txt", test_data)
        .await
        .unwrap();

    let mut xz = builder.into_inner().await.unwrap();
    xz.shutdown().await.unwrap();

    file
}
