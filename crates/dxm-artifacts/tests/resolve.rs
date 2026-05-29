use std::io::Cursor;

use async_tempfile::TempDir;
use dxm_artifacts::*;
use tokio::fs;
use wiremock::MockServer;

use crate::mocks::*;

mod mocks;

#[tokio::test]
async fn should_resolve_specific_build() {
    let version = BuildVersion::BuildNumber("3".to_owned());
    let (resolver, server) = setup_resolver("3").await;
    let _ = server.with_cfx_artifacts_check_windows("3").await;

    let build = resolver.resolve(&version).await.unwrap();

    assert_eq!(build.number(), "3");
    assert_eq!(build.commit_sha(), "xyz");
    assert_eq!(build.issues().count(), 4);
}

#[tokio::test]
async fn should_resolve_recommended_build() {
    let version = BuildVersion::Recommended;
    let (resolver, server) = setup_resolver("1").await;
    let _ = server.with_cfx_artifacts_check_windows("1").await;

    let build = resolver.resolve(&version).await.unwrap();

    assert_eq!(build.number(), "1");
    assert_eq!(build.commit_sha(), "xyz");
    assert_eq!(build.issues().count(), 1);
}

#[tokio::test]
async fn should_resolve_latest_build() {
    let version = BuildVersion::Latest;
    let (resolver, server) = setup_resolver("4").await;
    let _ = server
        .with_cfx_artifacts_check_windows("4")
        .await
        .with_cfx_artifacts_check_linux("4")
        .await;

    let build = resolver.resolve(&version).await.unwrap();

    assert_eq!(build.number(), "4");
    assert_eq!(build.commit_sha(), "xyz");
    assert_eq!(build.issues().count(), 1);
}

#[tokio::test]
async fn should_resolve_latest_jg_build() {
    let version = BuildVersion::LatestJg;
    let (resolver, server) = setup_resolver("2").await;
    let _ = server
        .with_cfx_artifacts_check_windows("2")
        .await
        .with_cfx_artifacts_check_linux("2")
        .await;

    let build = resolver.resolve(&version).await.unwrap();

    assert_eq!(build.number(), "2");
    assert_eq!(build.commit_sha(), "xyz");
    assert_eq!(build.issues().count(), 0);
}

#[tokio::test]
async fn should_fetch_windows() {
    let platform = ArtifactsPlatform::Windows;

    let (resolver, build, length, _) = setup_archive_resolver(platform).await;
    let dir = TempDir::new().await.unwrap();

    let result = resolver.fetch(&build, platform).await.unwrap();
    platform.extract(result.0, &dir).await.unwrap();

    assert_eq!(result.1, Some(length));
    assert_extract_contents(&dir).await;
}

#[tokio::test]
async fn should_fetch_linux() {
    let (resolver, build, length, _) = setup_archive_resolver(ArtifactsPlatform::Linux).await;
    let dir = TempDir::new().await.unwrap();

    let platform = ArtifactsPlatform::Linux;
    let bytes = resolver.fetch(&build, platform).await.unwrap();
    platform.extract(Cursor::new(bytes), &dir).await.unwrap();

    assert_extract_contents(&dir).await;
}

async fn setup_archive_resolver(
    platform: ArtifactsPlatform,
) -> (Artifacts, Build, u64, MockServer) {
    let version = BuildVersion::LatestJg;
    let build_number = "2";

    let (resolver, server) = setup_resolver(build_number).await;

    let archive = match platform {
        ArtifactsPlatform::Windows => setup_windows_archive().await,
        ArtifactsPlatform::Linux => setup_linux_archive().await,
        _ => unreachable!(),
    };

    let contents = fs::read(archive.file_path()).await.unwrap();
    let length = contents.len() as u64;

    let server = server
        .with_cfx_artifacts_windows(build_number, contents)
        .await;
    let build = resolver.resolve(&version).await.unwrap();

    (resolver, build, length, server)
}

async fn setup_resolver(build_number: &str) -> (Artifacts, MockServer) {
    let server = MockServer::start()
        .await
        .with_jg_artifacts_db()
        .await
        .with_cfx_changelogs_api()
        .await
        .with_github_git_ref(build_number)
        .await
        .with_github_git_tag()
        .await;
    let resolver = Artifacts::mock(&server);

    (resolver, server)
}
