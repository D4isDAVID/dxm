use dxm_init::{ENV_CFG_NAME, PERMISSIONS_CFG_NAME, README_NAME, RESOURCES_CFG_NAME, SECRETS_CFG_NAME, SERVER_CFG_NAME, TXDATA_CONFIG_NAME, TXDATA_DEFAULT_PROFILE, TXDATA_DIR, vcs::{GITIGNORE_NAME, TEMPLATE_EXTENSION, VcsOption}};
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
    let data_path = manifest.server.data(dir_path);

    assert!(dir_path.join(".git").exists());
    assert!(dir_path.join(GITIGNORE_NAME).exists());
    assert!(data_path.join(GITIGNORE_NAME).exists());
    assert!(data_path.join(ENV_CFG_NAME).with_added_extension(TEMPLATE_EXTENSION).exists());
    assert!(data_path.join(SECRETS_CFG_NAME).with_added_extension(TEMPLATE_EXTENSION).exists());
}

fn assert_init(dir_path: &std::path::Path) -> Manifest {
    let manifest = Manifest::read(dir_path).unwrap();
    let data_path = manifest.server.data(dir_path);
    let txdata_path = dir_path.join(TXDATA_DIR);
    let tx_profile_path = txdata_path.join(TXDATA_DEFAULT_PROFILE);

    assert!(dir_path.exists());
    assert!(dir_path.join(MANIFEST_NAME).exists());
    assert!(dir_path.join(README_NAME).exists());
    assert!(manifest.server.resources(dir_path).exists());
    assert!(data_path.exists());
    assert!(data_path.join(ENV_CFG_NAME).exists());
    assert!(data_path.join(PERMISSIONS_CFG_NAME).exists());
    assert!(data_path.join(RESOURCES_CFG_NAME).exists());
    assert!(data_path.join(SECRETS_CFG_NAME).exists());
    assert!(data_path.join(SERVER_CFG_NAME).exists());
    assert!(data_path.join(SERVER_CFG_NAME).exists());
    assert!(txdata_path.exists());
    assert!(tx_profile_path.exists());
    assert!(tx_profile_path.join(TXDATA_CONFIG_NAME).exists());

    manifest
}
