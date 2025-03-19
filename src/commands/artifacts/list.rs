//! Contains the command to list FXServer update channels.

use clap::Command;
use dxm_artifacts::{
    cfx::{ArtifactsChannel, ArtifactsPlatform, versions},
    jg::artifacts,
};

/// The command structure.
pub fn cli() -> Command {
    Command::new("list").about("List available FXServer artifacts")
}

/// The code ran when using the command.
pub fn execute() -> reqwest::Result<()> {
    let client = crate::util::reqwest::client().build()?;
    let platform = ArtifactsPlatform::default();
    let versions = versions(&client, &platform)?;
    let artifacts = artifacts(&client)?;

    print!(
        "\
latest-jg:  \t{}
latest:     \t{}
optional:   \t{}
recommended:\t{}
critical:   \t{}
",
        artifacts.alias_display(),
        versions.alias_display(&ArtifactsChannel::Latest),
        versions.alias_display(&ArtifactsChannel::Optional),
        versions.alias_display(&ArtifactsChannel::Recommended),
        versions.alias_display(&ArtifactsChannel::Critical),
    );

    Ok(())
}
