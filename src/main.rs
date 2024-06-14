use dxm::commands::{cli, execute};
use log::error;

fn main() -> anyhow::Result<()> {
    let args = cli().get_matches();

    if let Err(e) = execute(&args) {
        error!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
