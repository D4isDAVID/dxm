use std::path::PathBuf;

use clap::{Arg, ArgMatches, Command};

use crate::{context::CliContext, util::result::ResultUtil};

pub fn cli() -> Command {
    Command::new("run")
        .about("Start a dxm-managed server")
        .arg(
            Arg::new("server-path")
                .help("The path to the dxm-managed server")
                .value_parser(clap::value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("tx-profile")
                .help("When using txAdmin, the profile to use")
                .long("tx-profile")
                .short('t'),
        )
        .arg(
            Arg::new("server-args")
                .help("Extra args for FXServer")
                .num_args(0..)
                .last(true),
        )
}

pub fn execute(context: &mut CliContext, args: &ArgMatches) -> anyhow::Result<()> {
    let server_args = args
        .get_many::<String>("server-args")
        .map_or_else(Vec::new, Iterator::collect);

    let path = args
        .get_one::<PathBuf>("server-path")
        .map(dunce::canonicalize)
        .transpose()
        .prefix_err("invalid server path specified")?;

    context.find_manifest(path)?;
    let manifest = context.manifest()?;

    let base_path = context
        .paths()
        .manifest()
        .and_then(|p| p.parent())
        .expect("expected manifest path");
    let artifact = manifest.artifact();
    let server = manifest.server();

    match args.get_one::<String>("tx-profile") {
        Some(profile) => server.run_tx(base_path, artifact, profile, server_args)?,
        None => server.run(base_path, artifact, server_args)?,
    };

    log::info!("finished");
    Ok(())
}
