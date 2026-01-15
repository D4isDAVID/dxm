use dxm_init::{SERVER_CFG_NAME, vcs::VcsOption};
use dxm_manifest::{MANIFEST_NAME, Manifest};
use tempfile::tempdir;

#[test]
fn init_vcs_none() {
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    dxm_init::server(dir_path, &VcsOption::None).unwrap();

    assert_init(dir_path);
}

#[test]
fn init_vcs_git() {
    let dir = tempdir().unwrap();
    let dir_path = dir.path();

    dxm_init::server(dir_path, &VcsOption::Git).unwrap();

    let manifest = assert_init(dir_path);

    assert!(dir_path.join(".git").exists());
    assert!(dir_path.join(".gitignore").exists());
    assert!(manifest.server.data(dir_path).join(".gitignore").exists());
}

fn assert_init(dir_path: &std::path::Path) -> Manifest {
    let manifest = Manifest::read(dir_path).unwrap();
    let data_path = manifest.server.data(dir_path);

    assert!(dir_path.exists());
    assert!(dir_path.join(MANIFEST_NAME).exists());
    assert!(data_path.exists());
    assert!(data_path.join(SERVER_CFG_NAME).exists());
    assert!(manifest.server.resources(dir_path).exists());

    manifest
}
