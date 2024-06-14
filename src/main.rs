use dxm::commands;

fn main() -> anyhow::Result<()> {
    let args = commands::cli().get_matches();

    if let Err(e) = commands::execute(&args) {
        log::error!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
