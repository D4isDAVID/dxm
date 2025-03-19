//! Contains code for initializing VCS repositories and files.

use std::{error::Error, fmt::Display, path::Path, str::FromStr};

use git2::Repository;

const ROOT_GITIGNORE: &str = "\
# FXServer
/artifact/

# txAdmin
/txData/
";

const DATA_GITIGNORE: &str = "\
# Cache
/cache/

# KVP
/db/

# Miscellaneous
/.replxx_history
/imgui.ini
";

/// The possible version control systems to use in servers.
#[derive(Default, Clone)]
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
    pub fn init<P>(&self, path: P) -> Result<(), Box<dyn Error>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();

        match self {
            VcsOption::None => Ok(()),
            VcsOption::Git => {
                Repository::init(path)?;

                fs_err::write(path.join(".gitignore"), ROOT_GITIGNORE)?;
                fs_err::write(path.join("data").join(".gitignore"), DATA_GITIGNORE)?;

                Ok(())
            }
        }
    }
}
