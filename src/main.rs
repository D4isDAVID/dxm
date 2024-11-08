fn main() -> anyhow::Result<()> {
    let args = dxm::commands::cli().get_matches();

    if let Err(e) = dxm::commands::execute(&args) {
        log::error!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
