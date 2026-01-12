use std::{error::Error, path::Path};

use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use dxm_manifest::{Manifest, lockfile::Lockfile, sourcefile};
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
        let artifact_path = manifest.artifact.path(&manifest_path);
        let source_version = sourcefile::read(&artifact_path)?;

        if source_version.is_some_and(|v| v.trim() == version) {
            log::info!("artifact {} already installed", &version);

            return Ok(());
        }

        log::info!("installing artifact {}", &version);

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
        log::info!("artifact {} already installed", &version);

        return Ok(());
    }

    log::info!("updating to artifact {}", &version);

    dxm_artifacts::install(client, platform, version, &artifact_path)?;
    sourcefile::write(artifact_path, version)?;
    lockfile.set_artifact_version(version);

    log::info!("successfully updated artifact");

    Ok(())
}
