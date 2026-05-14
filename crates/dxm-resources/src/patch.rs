use std::{error::Error, path::Path};

mod git_cli;

const APPLIED_PATCH: &str = ".dxm.patch";

pub fn prepare<P, R>(patch_path: Option<P>, resource_path: R) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let resource_path = resource_path.as_ref();

    log::debug!("preparing {} for patch", resource_path.display());

    let mut repo = git_cli::TempGitRepo::init(resource_path)?;
    repo.reverse_applied_patch()?;

    repo.add()?;
    repo.commit()?;

    if let Some(patch_path) = patch_path {
        repo.apply(patch_path, false)?;
    }

    repo.keep();

    Ok(())
}

pub fn is_prepared<P>(resource_path: P) -> bool
where
    P: AsRef<Path>,
{
    resource_path.as_ref().join(".git").is_dir()
}

pub fn commit<P, R>(patch_path: P, resource_path: R) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    log::debug!("creating patch for {}", resource_path.display());

    let repo = git_cli::TempGitRepo::existing(resource_path);
    repo.add()?;

    let output = repo.diff()?;
    let patch = output.as_bytes();

    fs_err::write(resource_path.join(APPLIED_PATCH), patch)?;

    if let Some(patches_dir) = patch_path.parent() {
        fs_err::create_dir_all(patches_dir)?;
    }
    fs_err::write(patch_path, patch)?;

    Ok(())
}

pub fn is_pending<P, R>(patch_path: P, resource_path: R) -> std::io::Result<bool>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    Ok(
        match fs_err::read_to_string(resource_path.join(APPLIED_PATCH)) {
            Ok(applied) => {
                let patch = fs_err::read_to_string(patch_path)?;

                patch != applied
            }
            Err(err) => match err.kind() {
                std::io::ErrorKind::NotFound => true,
                _ => Err(err)?,
            },
        },
    )
}

pub fn apply<P, R>(patch_path: P, resource_path: R) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    let repo = git_cli::TempGitRepo::init(resource_path)?;
    repo.add()?;
    repo.commit()?;

    log::debug!(
        "applying patch {} to {}",
        patch_path.display(),
        resource_path.display()
    );

    repo.apply(patch_path, false)?;

    fs_err::copy(patch_path, resource_path.join(APPLIED_PATCH))?;

    Ok(())
}

pub fn remove<P, R>(patch_path: P, resource_path: R) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    let repo = git_cli::TempGitRepo::init(resource_path)?;
    repo.add()?;
    repo.commit()?;

    log::debug!(
        "removing patch {} from {}",
        patch_path.display(),
        resource_path.display()
    );

    repo.reverse_applied_patch()?;

    match fs_err::remove_file(patch_path) {
        Ok(r) => Ok(r),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Ok(()),
            _ => Err(err),
        },
    }?;

    match fs_err::remove_file(resource_path.join(APPLIED_PATCH)) {
        Ok(r) => Ok(r),
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => Ok(()),
            _ => Err(err),
        },
    }?;

    Ok(())
}
