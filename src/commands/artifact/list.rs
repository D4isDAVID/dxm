use clap::Command;

use crate::cfx::Cfx;

pub fn cli() -> Command {
    Command::new("list").about("List available FXServer artifacts")
}

pub fn execute() -> anyhow::Result<()> {
    let changelogs = Cfx::server_changelogs()?;

    print!(
        "\
latest:     \t{}
optional:   \t{}
recommended:\t{}
critical:   \t{}
",
        changelogs.latest(),
        changelogs.optional(),
        changelogs.recommended(),
        changelogs.critical()
    );

    Ok(())
}
