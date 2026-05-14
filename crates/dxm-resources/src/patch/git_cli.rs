use std::{
    error::Error,
    fmt::Display,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

const PATCH_IGNORE: &str = "\
# These are internal files, please do not change them
.dxm*
.gitignore
";

#[derive(Debug)]
struct GitNotInstalledError;

impl Display for GitNotInstalledError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "git must be installed to use this feature")?;

        Ok(())
    }
}

impl Error for GitNotInstalledError {}

#[derive(Debug)]
struct GitCommandError {
    subcommand: String,
    output: String,
}

impl Display for GitCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "git {} failed:", self.subcommand)?;

        for line in self.output.lines() {
            write!(f, "\n\t{line}")?;
        }

        Ok(())
    }
}

impl Error for GitCommandError {}

pub struct TempGitRepo {
    path: PathBuf,
    keep: bool,
}

impl Drop for TempGitRepo {
    fn drop(&mut self) {
        if self.keep {
            log::trace!("keeping temp repo {}", self.path.display());

            return;
        };

        log::trace!("dropping temp repo {}", self.path.display());

        if let Err(err) = fs_err::write(self.path.join(".gitignore"), crate::ROOT_GITIGNORE) {
            log::error!(
                "failed to clean up gitignore in {}: {err}",
                self.path.display()
            );
        }

        if let Err(err) = fs_err::remove_dir_all(self.path.join(".git")) {
            log::error!(
                "failed to clean up git repo in {}: {err}",
                self.path.display()
            );
        }
    }
}

impl TempGitRepo {
    pub fn init<P>(path: P) -> Result<Self, Box<dyn Error>>
    where
        P: Into<PathBuf>,
    {
        ensure_git_exists()?;

        let path = path.into();

        let repo = Self::existing(path);
        repo.run_git("init", |_| {})?;

        fs_err::write(repo.path.join(".gitignore"), PATCH_IGNORE)?;

        Ok(repo)
    }

    pub fn existing<P>(path: P) -> Self
    where
        P: Into<PathBuf>,
    {
        let path = path.into();

        Self { path, keep: false }
    }

    pub fn keep(&mut self) {
        self.keep = true;
    }

    pub fn add(&self) -> Result<(), Box<dyn Error>> {
        self.run_git("add", |command| {
            command.arg(".").arg("-A");
        })?;

        Ok(())
    }

    pub fn commit(&self) -> Result<(), Box<dyn Error>> {
        self.run_git("commit", |command| {
            command
                .arg("--allow-empty")
                .arg("--allow-empty-message")
                .arg("-m")
                .arg("");
        })?;

        Ok(())
    }

    pub fn diff(&self) -> Result<String, Box<dyn Error>> {
        self.run_git("diff", |command| {
            command
                .arg("--full-index")
                .arg("--text")
                .arg("--no-ext-diff")
                .arg("--no-color")
                .arg("--cached");
        })
    }

    pub fn apply<P>(&self, patch_path: P, reverse: bool) -> Result<String, Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let patch_path = patch_path.as_ref();

        self.run_git("apply", |command| {
            if reverse {
                command.arg("--reverse");
            }

            command.arg("--allow-empty").arg(de_unc(patch_path));
        })
    }

    pub fn reverse_applied_patch(&self) -> Result<(), Box<dyn Error>> {
        let applied_path = self.path.join(super::APPLIED_PATCH);

        if fs_err::exists(&applied_path)? {
            self.apply(applied_path, true)?;
        };

        Ok(())
    }

    fn run_git<S, F>(&self, subcommand: S, func: F) -> Result<String, Box<dyn Error>>
    where
        S: AsRef<str>,
        F: FnOnce(&mut Command),
    {
        let subcommand = subcommand.as_ref();

        let mut command = Command::new("git");
        command
            .current_dir(&self.path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("HOME", "")
            .env("XDG_CONFIG_HOME", "")
            .env("USERPROFILE", "")
            .arg("-c")
            .arg("core.safecrlf=false")
            .arg("-c")
            .arg("init.defaultBranch=main")
            .arg("-c")
            .arg("user.email=dxm@d4vid.dev")
            .arg("-c")
            .arg("user.name=dxm")
            .arg(subcommand);

        func(&mut command);

        log::debug!("running git command {command:?}");

        let output = command.spawn()?.wait_with_output()?;
        let stderr = String::from_utf8(output.stderr)?;

        if !stderr.is_empty() {
            Err(GitCommandError {
                subcommand: subcommand.to_owned(),
                output: stderr,
            })?;
        }

        Ok(String::from_utf8(output.stdout)?)
    }
}

fn ensure_git_exists() -> Result<(), Box<dyn Error>> {
    if !Command::new("git")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .arg("-v")
        .spawn()?
        .wait()?
        .success()
    {
        Err(GitNotInstalledError)?;
    }

    Ok(())
}

fn de_unc<P>(path: P) -> String
where
    P: AsRef<Path>,
{
    let path = path.as_ref().as_os_str().to_string_lossy();

    path.strip_prefix("\\\\?\\")
        .map(|s| s.to_owned())
        .unwrap_or(path.to_string())
}
