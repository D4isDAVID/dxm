use std::{error::Error, path::Path, path::PathBuf};

use dxm_artifacts::cfx::ArtifactsPlatform;
use dxm_init::vcs::VcsOption;
use dxm_manifest::lockfile::Lockfile;

pub fn server<P>(
    path: P,
    vcs: &VcsOption,
    artifact_path: Option<&PathBuf>,
) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    log::info!("creating files");
    dxm_init::server(path, vcs)?;

    let (manifest_path, mut manifest) = crate::util::manifest::find(path)?;
    let mut lockfile = Lockfile::read(&manifest_path)?;

    let client = crate::util::reqwest::github_client().build()?;
    let platform = ArtifactsPlatform::default();

    if let Some(artifact_path) = artifact_path {
        manifest.artifact.set_path("", &artifact_path)?;
        manifest.write_artifact(&manifest_path)?;
    } else {
        crate::util::artifacts::install(
            &client,
            &platform,
            &manifest_path,
            &manifest,
            &mut lockfile,
        )?;
    }

    crate::util::resources::install(&client, &manifest_path, &manifest, &mut lockfile)?;
    lockfile.write(manifest_path)?;

    Ok(())
}
