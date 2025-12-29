use std::{error::Error, path::Path};

use dxm_manifest::{Manifest, lockfile::Lockfile};
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

        log::info!("installing resource {}", resource_name);

        let resource_url =
            dxm_resources::install(client, url, base_path, resource_name, nested_path)?;

        lockfile.set_resource_url(resource_name, resource_url);
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

        log::info!("updating resource {}", resource_name);

        let resource_url =
            dxm_resources::update(client, url, base_path, resource_name, nested_path)?;

        lockfile.set_resource_url(resource_name, resource_url);
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

    log::info!("uninstalling resource {}", resource_name);

    dxm_resources::uninstall(base_path, resource_name)?;

    lockfile.remove_resource_url(resource_name);

    Ok(())
}
