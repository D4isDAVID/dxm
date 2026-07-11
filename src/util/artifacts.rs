use std::{error::Error, path::Path};

use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use dxm_manifest::{
    Manifest, artifact::Artifact, lockfile::Lockfile, resource::Resource, sourcefile,
};
use reqwest::blocking::Client;

pub const MONITOR_RESOURCE: &str = "monitor";

pub fn install<P>(
    client: &Client,
    platform: &ArtifactsPlatform,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    if let Some(version) = lockfile.artifact_version() {
        let artifact_path = manifest.artifact.path(&manifest_path);
        let source_version = sourcefile::read(&artifact_path)?;

        if source_version.is_some_and(|v| v.trim() == version) {
            log::info!("artifact {} already installed", version);

            return Ok(());
        }

        log::info!("installing artifact {}", version);

        dxm_artifacts::install(client, platform, version, &artifact_path)?;
        sourcefile::write(artifact_path, version)?;

        log::info!("successfully installed artifact");
    } else {
        update(client, platform, &manifest_path, manifest, lockfile)?;
    };

    Ok(())
}

pub fn update<P>(
    client: &Client,
    platform: &ArtifactsPlatform,
    manifest_path: P,
    manifest: &Manifest,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let artifact = &manifest.artifact;
    let artifact_path = artifact.path(&manifest_path);

    let version = if let Some(channel) = artifact.channel() {
        log::info!("getting versions");

        &if channel == ArtifactsChannel::LatestJg {
            dxm_artifacts::jg::artifacts(client)?.version().to_owned()
        } else {
            dxm_artifacts::cfx::versions(client, platform)?
                .version(&channel)
                .to_owned()
        }
    } else {
        artifact.version()
    };

    let lockfile_updated = lockfile.artifact_version().is_some_and(|v| v == version);
    let sourcefile_updated = sourcefile::read(&artifact_path)?.is_some_and(|v| v == version);

    if lockfile_updated && sourcefile_updated {
        log::info!("artifact {} already installed", version);

        return Ok(());
    }

    let vacated_monitor = manifest
        .artifact
        .monitor()
        .map(|_| {
            dxm_resources::VacatedDir::temp(
                manifest
                    .artifact
                    .system_resources(manifest_path, platform)
                    .join(MONITOR_RESOURCE),
            )
        })
        .transpose()?
        .flatten();

    log::info!("updating to artifact {}", version);

    dxm_artifacts::install(client, platform, version, &artifact_path)?;
    sourcefile::write(artifact_path, version)?;
    lockfile.set_artifact_version(version);

    if let Some(vacated_monitor) = vacated_monitor {
        vacated_monitor.bring_back()?;
    }

    log::info!("successfully updated artifact");

    Ok(())
}

pub fn install_monitor<P>(
    client: &Client,
    manifest_path: P,
    artifact: &Artifact,
    platform: &ArtifactsPlatform,
    resource: &Resource,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let manifest_path = manifest_path.as_ref();

    let resources_path = artifact.system_resources(manifest_path, platform);

    let vacated_monitor = dxm_resources::VacatedDir::new(
        resources_path.join(MONITOR_RESOURCE),
        artifact.tx_monitor(manifest_path, platform),
    )?;

    let result = crate::util::resources::install_single(
        client,
        manifest_path,
        resources_path,
        resource,
        lockfile.monitor_url(),
        MONITOR_RESOURCE,
    );

    if result.is_err()
        && let Some(vacated_monitor) = vacated_monitor
    {
        vacated_monitor.bring_back()?;
    }

    let lock_url = result?;

    if let Some(lock_url) = lock_url {
        lockfile.set_monitor_url(lock_url);

        log::info!("successfully installed third-party monitor");
    }

    Ok(())
}

pub fn update_monitor<P>(
    client: &Client,
    manifest_path: P,
    artifact: &Artifact,
    platform: &ArtifactsPlatform,
    resource: &Resource,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let manifest_path = manifest_path.as_ref();

    let resources_path = artifact.system_resources(manifest_path, platform);

    let lock_url = crate::util::resources::update_single(
        client,
        manifest_path,
        resources_path,
        resource,
        lockfile.monitor_url(),
        MONITOR_RESOURCE,
    )?;

    if let Some(lock_url) = lock_url {
        lockfile.set_monitor_url(lock_url);

        log::info!("successfully updated third-party monitor");
    }

    Ok(())
}

pub fn remove_monitor<P>(
    manifest_path: P,
    artifact: &Artifact,
    platform: &ArtifactsPlatform,
    resource: &Resource,
    lockfile: &mut Lockfile,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let manifest_path = manifest_path.as_ref();

    let resources_path = artifact.system_resources(manifest_path, platform);

    crate::util::resources::uninstall_single(&resources_path, resource, MONITOR_RESOURCE)?;

    lockfile.remove_monitor_url();

    dxm_resources::VacatedDir::new(
        artifact.tx_monitor(manifest_path, platform),
        resources_path.join(MONITOR_RESOURCE),
    )?;

    log::info!("successfully removed third-party monitor");

    Ok(())
}
