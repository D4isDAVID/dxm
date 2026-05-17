//! Contains code for initializing VCS repositories and files.

use std::{error::Error, fmt::Display, path::Path, str::FromStr};

use dxm_manifest::Manifest;
use git2::Repository;

pub const GITIGNORE_NAME: &str = ".gitignore";
pub const TEMPLATE_EXTENSION: &str = "example";

const GIT_README: &str = include_str!("../templates/git/README.md");
const ROOT_GITIGNORE: &str = include_str!("../templates/git/root.gitignore");
const DATA_GITIGNORE: &str = include_str!("../templates/git/data.gitignore");

/// The possible version control systems to use in servers.
#[derive(Default, Debug, PartialEq, Eq, Clone)]
pub enum VcsOption {
    #[default]
    None,
    Git,
}

#[derive(Debug)]
pub struct ParseVcsOptionError {
    option: String,
}

impl Display for ParseVcsOptionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "unknown vsc option {}", &self.option)?;

        Ok(())
    }
}

impl Error for ParseVcsOptionError {}

impl FromStr for VcsOption {
    type Err = ParseVcsOptionError;

    fn from_str(option: &str) -> Result<Self, Self::Err> {
        match option {
            "none" => Ok(Self::None),
            "git" => Ok(Self::Git),
            _ => Err(ParseVcsOptionError {
                option: option.to_owned(),
            }),
        }
    }
}

impl VcsOption {
    /// Initialize the VCS repository and files.
    pub fn init<P>(&self, path: P, manifest: &Manifest) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let data_path = manifest.server.data(path);

        match self {
            VcsOption::None => Ok(()),
            VcsOption::Git => {
                Repository::init(path)?;

                fs_err::write(crate::README_NAME, GIT_README)?;
                fs_err::write(path.join(GITIGNORE_NAME), ROOT_GITIGNORE)?;
                fs_err::write(data_path.join(GITIGNORE_NAME), DATA_GITIGNORE)?;

                create_template(data_path.join(crate::ENV_CFG_NAME))?;
                create_template(data_path.join(crate::SECRETS_CFG_NAME))?;

                Ok(())
            }
        }
    }
}

fn create_template<P>(path: P) -> std::io::Result<u64>
where
    P: AsRef<Path>,
{
    let path = path.as_ref();

    fs_err::copy(path, path.with_added_extension(TEMPLATE_EXTENSION))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_returns_value_when_valid() {
        assert_eq!(VcsOption::from_str("none").unwrap(), VcsOption::None);
    }

    #[test]
    #[should_panic]
    fn parse_returns_error_when_invalid() {
        VcsOption::from_str("").unwrap();
    }
}
