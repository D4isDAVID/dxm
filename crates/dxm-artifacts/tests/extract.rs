use async_tempfile::TempDir;
use dxm_artifacts::*;
use tokio::io::BufReader;

use crate::mocks::*;

mod mocks;

#[tokio::test]
async fn should_extract_windows() {
    let archive = setup_windows_archive().await;
    let dir = TempDir::new().await.unwrap();

    ArtifactsPlatform::Windows
        .extract(BufReader::new(archive.open_ro().await.unwrap()), &dir)
        .await
        .unwrap();

    assert_extract_contents(&dir).await;
}

#[tokio::test]
async fn should_extract_linux() {
    let archive = setup_linux_archive().await;
    let dir = TempDir::new().await.unwrap();

    ArtifactsPlatform::Linux
        .extract(BufReader::new(archive.open_ro().await.unwrap()), &dir)
        .await
        .unwrap();

    assert_extract_contents(&dir).await;
}
