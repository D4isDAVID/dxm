use std::{error::Error, path::Path};

use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use dxm_manifest::{Manifest, lockfile::Lockfile};

pub fn update<P>(path: P, manifest: &Manifest) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let artifact = &manifest.artifact;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    if let Some(channel) = artifact.channel() {
        log::info!("getting versions");

        let version = if channel == ArtifactsChannel::LatestJg {
            dxm_artifacts::jg::artifacts(&client)?.version().to_owned()
        } else {
            dxm_artifacts::cfx::versions(&client, &platform)?
                .version(&channel)
                .to_owned()
        };

        log::info!("installing artifact {}", &version);
        dxm_artifacts::install(&client, &platform, &version, artifact.path(&path))?;

        Lockfile::write_artifact_version(path, version)?;

        log::info!("successfully updated artifact");
    } else {
        let version = artifact.version();

        log::info!("installing artifact {}", &version);
        dxm_artifacts::install(&client, &platform, &version, artifact.path(&path))?;

        Lockfile::write_artifact_version(path, version)?;

        log::info!("successfully updated artifact");
    }

    Ok(())
}
