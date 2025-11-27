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
        let artifact_url =
            dxm_artifacts::install(&client, &platform, &version, artifact.path(&path))?;

        Lockfile::write_artifact_url(path, artifact_url)?;

        log::info!("successfully updated artifact");
    } else {
        log::error!("version in manifest is set to a static version");
    }

    Ok(())
}
