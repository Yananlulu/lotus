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

    if let Some(_) = matches.subcommand_matches("generate:config") {
        pug::app::generate::config::run(cfg)?;
        return Ok(());
    }

    let cfg = pug::env::Config::new(cfg)?;
    if let Some(_) = matches.subcommand_matches("generate:nginx") {
        pug::app::generate::nginx::run(
            "aaa".to_string(),
            cfg.http.port,
            matches.is_present(pug::app::generate::nginx::SSL),
        )?;
        return Ok(());
    }

    let app = cfg.rocket()?;
    app.launch();
    Ok(())
}
