use std::result::Result as StdResult;

use pug;

use super::{
    env::{self, Config},
    errors::{Error, Result},
};

pub struct Server {}

impl pug::app::Server for Server {
    type Config = Config;
    type Error = Error;
    fn run(&self, cfg: &Self::Config) -> StdResult<(), Self::Error> {
        pug::rocket::custom(cfg.pug.rocket()?)
            // .mount("/", routes![index])
            .launch();;
        Ok(())
    }
}

pub fn run() -> Result<()> {
    let version = env::version();
    let app = pug::app::App::new(
        env::NAME,
        &version[..],
        Some(env::AUTHORS),
        Some(env::DESCRIPTION),
        Some(env::BANNER),
        Some(env::HOMEPAGE),
    );
    app.run(
        &Server {},
        &vec![pug::i18n::locales::migration(), pug::settings::migration()],
    )
}
