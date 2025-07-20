use std::{error::Error, path::Path};

use dxm_artifacts::cfx::{ArtifactsChannel, ArtifactsPlatform};
use dxm_manifest::Manifest;

pub fn update<P>(path: P, manifest: &mut Manifest) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let artifact = &mut manifest.artifact;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    log::info!("getting versions");

    let channel = artifact.channel();
    let version = if channel == ArtifactsChannel::LatestJg {
        dxm_artifacts::jg::artifacts(&client)?.version().to_owned()
    } else {
        dxm_artifacts::cfx::versions(&client, &platform)?
            .version(&channel)
            .to_owned()
    };

    log::info!("installing artifact {}", &version);
    dxm_artifacts::install(&client, &platform, &version, artifact.path(&path))?;

    artifact.set_version(version);
    manifest.write(path)?;

    log::info!("successfully updated artifact");

    Ok(())
}
