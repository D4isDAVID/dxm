use std::{error::Error, path::Path};

use tempfile::TempDir;

const GIT_DEFAULT_USERNAME: &str = "git";

pub fn clone<S, R, P, N>(
    url: S,
    rev: R,
    path: P,
    nested_path: N,
) -> Result<Option<String>, Box<dyn Error>>
where
    S: AsRef<str>,
    R: AsRef<str>,
    P: AsRef<Path>,
    N: AsRef<Path>,
{
    let url = url.as_ref();
    let rev = rev.as_ref();
    let path = path.as_ref();
    let nested_path = nested_path.as_ref();

    let config = git2::Config::open_default()?;

    let mut remote_callbacks = git2::RemoteCallbacks::new();
    remote_callbacks.credentials(credentials(&config));

    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);
    fetch_options.download_tags(git2::AutotagOption::None);
    fetch_options.depth(1);

    let mut repo_builder = git2::build::RepoBuilder::new();
    repo_builder.fetch_options(fetch_options);

    let dir = TempDir::with_suffix("dxm-resource")?;
    let temp_path = dir.path();
    log::debug!("cloning repository {}", url);
    let repository = repo_builder.clone(url, temp_path)?;

    if !rev.is_empty() {
        log::debug!("parsing out revision {rev:?}");

        let commit = repository.revparse_single(rev)?.peel_to_commit()?;

        log::debug!("checkout out commit {}", commit.id());

        repository.checkout_tree(commit.as_object(), None)?;
        repository.set_head_detached(commit.id())?;
    }

    let rev = if rev.is_empty() {
        Some(
            repository
                .head()?
                .resolve()?
                .peel_to_commit()?
                .id()
                .to_string(),
        )
    } else {
        None
    };

    drop(repository);

    log::trace!("removing .git directory");
    fs_err::remove_dir_all(temp_path.join(".git"))?;

    log::debug!("moving contents into {}", path.display());
    crate::move_dir_contents(temp_path.join(nested_path), path)?;

    Ok(rev)
}

fn credentials(
    config: &git2::Config,
) -> impl FnMut(&str, Option<&str>, git2::CredentialType) -> Result<git2::Cred, git2::Error> {
    let env_username = std::env::var("USER").or_else(|_| std::env::var("USERNAME"));

    let mut attempted_ssh_agent = false;

    move |url, username, allowed| {
        log::trace!("getting git credentials");

        if allowed.contains(git2::CredentialType::USERNAME) {
            log::trace!("trying to wrap username");

            return git2::Cred::username(env_username.as_deref().unwrap_or(GIT_DEFAULT_USERNAME))
                .inspect_err(|err| log::debug!("failed to wrap username: {err}"));
        }

        if allowed.contains(git2::CredentialType::SSH_KEY)
            && let Some(username) = username
            && !attempted_ssh_agent
        {
            log::trace!("trying to use ssh agent");

            attempted_ssh_agent = true;

            return git2::Cred::ssh_key_from_agent(username)
                .inspect_err(|err| log::debug!("failed using ssh agent: {err}"));
        }

        if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            log::trace!("trying to use credential helper");

            match git2::Cred::credential_helper(config, url, username) {
                Ok(cred) => return Ok(cred),
                Err(err) => log::debug!("failed using credential helper: {err}"),
            };
        }

        Err(git2::Error::from_str("git authentication failed"))
    }
}
