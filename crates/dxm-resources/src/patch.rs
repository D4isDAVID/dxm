use std::{
    error::Error,
    path::Path,
    process::{Command, Stdio},
};

const OLD_CONTENT_DIR: &str = ".dxm-patches";
const APPLIED_PATCH: &str = ".dxm.patch";

pub fn prepare<S, M, R>(name: S, manifest_path: M, resource_path: R) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
    M: AsRef<Path>,
    R: AsRef<Path>,
{
    let name = name.as_ref();
    let old_path = manifest_path.as_ref().join(OLD_CONTENT_DIR).join(name);
    let resource_path = resource_path.as_ref();

    log::debug!("preparing {} for patch", resource_path.display());

    fs_err::create_dir_all(&old_path)?;

    fs_extra::dir::copy(
        resource_path,
        &old_path,
        &fs_extra::dir::CopyOptions::new().content_only(true),
    )?;

    // TODO: revert applied patch in old_path

    Ok(())
}

pub fn is_prepared<S, P>(name: S, manifest_path: P) -> bool
where
    S: AsRef<str>,
    P: AsRef<Path>,
{
    let name = name.as_ref();
    let old_path = manifest_path.as_ref().join(OLD_CONTENT_DIR).join(name);

    old_path.is_dir()
}

pub fn commit<S, M, P, R>(
    name: S,
    manifest_path: M,
    patch_path: P,
    resource_path: R,
) -> Result<(), Box<dyn Error>>
where
    S: AsRef<str>,
    M: AsRef<Path>,
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let name = name.as_ref();
    let old_container = manifest_path.as_ref().join(OLD_CONTENT_DIR);
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    let old_path = old_container.join(name);

    log::debug!("creating patch for {}", resource_path.display());

    // TODO: create patch
    let output = "";

    if let Some(patches_dir) = patch_path.parent() {
        fs_err::create_dir_all(patches_dir)?;
    }
    fs_err::write(patch_path, output.as_bytes())?;
    fs_err::write(resource_path.join(APPLIED_PATCH), output.as_bytes())?;

    fs_err::remove_dir_all(old_path)?;

    if !crate::is_dir_with_files(&old_container)? {
        fs_err::remove_dir(old_container)?;
    }

    Ok(())
}

pub fn apply<P, R>(patch_path: P, resource_path: R, reverse: bool) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    // TODO: revert applied patch

    if !reverse {
        log::debug!(
            "applying patch {} to {}",
            patch_path.display(),
            resource_path.display()
        );

        let patch = fs_err::read_to_string(patch_path)?;

        // TODO: apply patch

        fs_err::write(resource_path.join(APPLIED_PATCH), &patch)?;
    }

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

pub fn remove<P, R>(patch_path: P, resource_path: R) -> Result<(), Box<dyn Error>>
where
    P: AsRef<Path>,
    R: AsRef<Path>,
{
    let patch_path = patch_path.as_ref();
    let resource_path = resource_path.as_ref();

    log::debug!(
        "removing patch {} from {}",
        patch_path.display(),
        resource_path.display()
    );

    apply(patch_path, resource_path, true)?;

    fs_err::remove_file(patch_path)?;

    Ok(())
}
