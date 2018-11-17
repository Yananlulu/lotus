use pug::{self, clap::SubCommand};

use super::{env, errors::Result};

pub fn run() -> Result<()> {
    let cfg = "config.toml";
    let version = env::version();
    let matches = pug::app::new(
        env::NAME,
        &version[..],
        Some(env::AUTHORS),
        Some(env::DESCRIPTION),
        Some(env::BANNER),
        Some(env::HOMEPAGE),
    )
    .subcommand(pug::app::generate::nginx::command())
    .subcommand(
        SubCommand::with_name(pug::app::generate::config::NAME)
            .about(&*pug::app::generate::config::help(cfg)),
    )
    .get_matches();

    if let Some(_) = matches.subcommand_matches(pug::app::generate::config::NAME) {
        pug::app::generate::config::run::<&'static str, pug::env::Config>(cfg)?;
        return Ok(());
    }

    info!("load configuration from {}", cfg);
    let cfg: pug::env::Config = pug::parser::toml(cfg)?;
    if let Some(matches) = matches.subcommand_matches(pug::app::generate::nginx::COMMAND_NAME) {
        let name = matches
            .value_of(pug::app::generate::nginx::ARG_SERVER_NAME)
            .unwrap();
        pug::app::generate::nginx::run(
            name.to_string(),
            cfg.http.port,
            matches.is_present(pug::app::generate::nginx::ARG_HTTPS),
        )?;
        return Ok(());
    }

    Ok(())
}
