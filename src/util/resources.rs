use std::{error::Error, fs, path::Path};

use dxm_manifest::{Manifest, lockfile::Lockfile, sourcefile};
use reqwest::blocking::Client;

pub fn install<P>(
    client: &Client,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    for resource_name in manifest.resources.keys() {
        install_single(client, &manifest_path, manifest, lockfile, resource_name)?;
    }

    Ok(())
}

pub fn update<P>(
    client: &Client,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    for resource_name in manifest.resources.keys() {
        update_single(client, &manifest_path, manifest, lockfile, resource_name)?;
    }

    Ok(())
}

pub fn install_single<P, S>(
    client: &Client,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
    resource_name: S,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let manifest_path = manifest_path.as_ref();
    let resource_name = resource_name.as_ref();

    let resource = manifest
        .resources
        .get(resource_name)
        .unwrap_or_else(|| panic!("no such resource {}", resource_name));

    if let Some(url) = lockfile.get_resource_url(resource_name).or(resource.url()) {
        let resources_path = &manifest.server.resources(manifest_path);
        let base_path = resource.category(resources_path);
        let nested_path = resource.nested_path();
        let resource_path = base_path.join(resource_name);

        let url = dxm_resources::resolve_download_url(client, url)?;
        let source_url = sourcefile::read(base_path.join(resource_name))?;

        if source_url.is_some_and(|u| u == url) {
            log::info!("resource {} already installed", resource_name);

            return Ok(());
        }

        log::info!("installing resource {}", resource_name);

        dxm_resources::install(client, &url, &resource_path, nested_path)?;

        sourcefile::write(base_path.join(resource_name), &url)?;
        lockfile.set_resource_url(resource_name, url);
    } else {
        log::warn!("no download url found for {}", resource_name);
    };

    Ok(())
}

pub fn update_single<P, S>(
    client: &Client,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
    resource_name: S,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let manifest_path = manifest_path.as_ref();
    let resource_name = resource_name.as_ref();

    let resource = manifest
        .resources
        .get(resource_name)
        .unwrap_or_else(|| panic!("no such resource {}", resource_name));

    if let Some(url) = resource.url() {
        let resources_path = &manifest.server.resources(manifest_path);
        let base_path = resource.category(resources_path);
        let nested_path = resource.nested_path();
        let resource_path = base_path.join(resource_name);

        let url = dxm_resources::resolve_download_url(client, url)?;
        let lockfile_updated = lockfile
            .get_resource_url(resource_name)
            .is_some_and(|u| u == url);
        let sourcefile_updated =
            sourcefile::read(base_path.join(resource_name))?.is_some_and(|u| u == url);

        if lockfile_updated && sourcefile_updated {
            log::info!("resource {} already updated", resource_name);

            return Ok(());
        }

        log::info!("updating resource {}", resource_name);

        dxm_resources::install(client, &url, &resource_path, nested_path)?;

        sourcefile::write(base_path.join(resource_name), &url)?;
        lockfile.set_resource_url(resource_name, url);
    } else {
        log::warn!("no download url found for {}", resource_name);
    };

    Ok(())
}

pub fn uninstall_single<P, S>(
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
    resource_name: S,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let manifest_path = manifest_path.as_ref();
    let resource_name = resource_name.as_ref();

    let resource = manifest
        .resources
        .get(resource_name)
        .unwrap_or_else(|| panic!("no such resource {}", resource_name));

    let resources_path = &manifest.server.resources(manifest_path);
    let base_path = resource.category(resources_path);
    let resource_path = base_path.join(resource_name);

    log::info!("uninstalling resource {}", resource_name);

    fs::remove_dir_all(resource_path)?;

    lockfile.remove_resource_url(resource_name);

    Ok(())
}
