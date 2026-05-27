use std::path::PathBuf;

use async_tempfile::TempDir;
use dxm_init::{Template, TemplateError};
use tokio::fs;

#[tokio::test]
pub async fn should_write_template() {
    let dir = TempDir::new().await.unwrap();
    Template::new()
        .file("test.txt", "test")
        .write(&dir)
        .await
        .unwrap();

    let contents = fs::read(dir.join("test.txt")).await.unwrap();

    assert_eq!(contents, b"test");
}

#[tokio::test]
pub async fn should_return_error_for_invalid_path() {
    let path = PathBuf::from("../test.txt");

    let dir = TempDir::new().await.unwrap();
    let result = Template::new().file(path, "test").write(dir).await;

    assert!(result.is_err_and(|e| matches!(e, TemplateError::InvalidPath(_))))
}

#[tokio::test]
pub async fn should_return_error_for_absolute_path() {
    #[cfg(windows)]
    let path = PathBuf::from("c:\\test.txt");
    #[cfg(unix)]
    let path = PathBuf::from("/test.txt");

    let dir = TempDir::new().await.unwrap();
    let result = Template::new().file(path, "test").write(dir).await;

    assert!(result.is_err_and(|e| matches!(e, TemplateError::InvalidPath(_))))
}
