use std::{error::Error, fs, path::Path};

use dxm_manifest::{Manifest, lockfile::Lockfile, resource::Resource, sourcefile};
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
    let manifest_path = manifest_path.as_ref();

    let resources_path = manifest.server.resources(manifest_path);

    for (resource_name, resource) in manifest.resources.iter() {
        let lock_url = install_single(
            client,
            manifest_path,
            &resources_path,
            resource,
            lockfile.get_resource_url(resource_name),
            resource_name,
        )?;

        if let Some(lock_url) = lock_url {
            lockfile.set_resource_url(resource_name, lock_url);
        }
    }

    Ok(())
}

pub fn update<P>(
    client: &Client,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
    resource_names: Vec<&String>,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let manifest_path = manifest_path.as_ref();

    let resources_path = manifest.server.resources(manifest_path);

    let iter = if resource_names.is_empty() {
        manifest.resources.keys().collect()
    } else {
        resource_names
    };

    for resource_name in iter {
        let Some(resource) = manifest.resources.get(resource_name) else {
            log::warn!("no such resource {resource_name}");

            continue;
        };

        let lock_url = update_single(
            client,
            manifest_path,
            &resources_path,
            resource,
            lockfile.get_resource_url(resource_name),
            resource_name,
        )?;

        if let Some(lock_url) = lock_url {
            lockfile.set_resource_url(resource_name, lock_url);
        }
    }

    Ok(())
}

pub fn install_single<M, P, S>(
    client: &Client,
    manifest_path: M,
    resources_path: P,
    resource: &Resource,
    lock_url: Option<&str>,
    resource_name: S,
) -> Result<Option<String>, Box<dyn Error>>
where
    M: AsRef<Path>,
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let manifest_path = manifest_path.as_ref();
    let resources_path = resources_path.as_ref();
    let resource_name = resource_name.as_ref();

    if let Some(url) = lock_url.or(resource.url()) {
        let base_path = resource.category(resources_path);
        let nested_path = resource.nested_path();
        let resource_path = base_path.join(resource_name);

        log::debug!("resolving resource url {}", url);

        let source = dxm_resources::resolve(client, url)?;
        let url = source.url();
        log::debug!("resolved resource url to {}", source);

        let source_url = sourcefile::read(&resource_path)?;
        if source_url.is_some_and(|u| u == url) {
            log::info!("resource {} already installed", resource_name);

            patch(manifest_path, resource_path, resource, resource_name)?;

            return Ok(None);
        }

        log::info!("installing resource {}", resource_name);

        let url = dxm_resources::install(&source, &resource_path, nested_path)?.unwrap_or(url);
        sourcefile::write(&resource_path, &url)?;

        patch(manifest_path, resource_path, resource, resource_name)?;

        Ok(Some(url))
    } else {
        log::warn!("no download url found for {}", resource_name);

        Ok(None)
    }
}

pub fn update_single<M, P, S>(
    client: &Client,
    manifest_path: M,
    resources_path: P,
    resource: &Resource,
    lock_url: Option<&str>,
    resource_name: S,
) -> Result<Option<String>, Box<dyn Error>>
where
    M: AsRef<Path>,
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let manifest_path = manifest_path.as_ref();
    let resources_path = resources_path.as_ref();
    let resource_name = resource_name.as_ref();

    if let Some(url) = resource.url() {
        let base_path = resource.category(resources_path);
        let nested_path = resource.nested_path();
        let resource_path = base_path.join(resource_name);

        let source = dxm_resources::resolve(client, url)?;
        let url = source.url();
        log::debug!("resolved resource url to {}", url);

        let lockfile_updated = lock_url.is_some_and(|u| u == url);
        let sourcefile_updated = sourcefile::read(&resource_path)?.is_some_and(|u| u == url);

        if lockfile_updated && sourcefile_updated {
            log::info!("resource {} already updated", resource_name);

            patch(manifest_path, resource_path, resource, resource_name)?;

            return Ok(None);
        }

        log::info!("updating resource {}", resource_name);

        let url = dxm_resources::install(&source, &resource_path, nested_path)?.unwrap_or(url);
        sourcefile::write(&resource_path, &url)?;

        patch(manifest_path, resource_path, resource, resource_name)?;

        Ok(Some(url))
    } else {
        log::warn!("no download url found for {}", resource_name);
        Ok(None)
    }
}

pub fn uninstall_single<P, S>(
    resources_path: P,
    resource: &Resource,
    resource_name: S,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    let resources_path = resources_path.as_ref();
    let resource_name = resource_name.as_ref();

    let base_path = resource.category(resources_path);
    let resource_path = base_path.join(resource_name);

    log::info!("uninstalling resource {}", resource_name);

    fs::remove_dir_all(resource_path)?;

    Ok(())
}

fn patch<M, R, S>(
    manifest_path: M,
    resource_path: R,
    resource: &Resource,
    resource_name: S,
) -> Result<(), Box<dyn Error>>
where
    M: AsRef<Path>,
    R: AsRef<Path>,
    S: AsRef<str>,
{
    if let Some(patch) = resource.patch(manifest_path)
        && dxm_resources::patch::is_pending(&patch, &resource_path)?
    {
        log::info!("patching resource {}", resource_name.as_ref());

        match dxm_resources::patch::apply(patch, resource_path) {
            Ok(_) => log::info!("patched resource {}", resource_name.as_ref()),
            Err(err) => {
                log::error!("failed to patch resource, make sure the patch isn't outdated! {err}")
            }
        }
    }

    Ok(())
}
