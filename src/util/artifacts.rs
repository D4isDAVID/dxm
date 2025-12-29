use std::{error::Error, path::Path};

use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use dxm_manifest::{Manifest, lockfile::Lockfile};
use reqwest::blocking::Client;

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
        log::info!("installing artifact {}", &version);

        dxm_artifacts::install(
            client,
            platform,
            version,
            manifest.artifact.path(&manifest_path),
        )?;

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

    log::info!("updating to artifact {}", &version);

    dxm_artifacts::install(client, platform, version, artifact.path(&manifest_path))?;
    lockfile.set_artifact_version(version);

    log::info!("successfully updated artifact");

    Ok(())
}
