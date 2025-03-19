use std::error::Error;

use dxm::commands::ExecuteOptions;

fn main() -> Result<(), Box<dyn Error>> {
    dxm::log::init()?;

    let args = dxm::commands::cli().get_matches();

    if let Err(e) = dxm::commands::execute(&args, &ExecuteOptions::default()) {
        log::error!("{e}");
        std::process::exit(1);
    }

    Ok(())
}
