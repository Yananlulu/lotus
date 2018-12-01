use pug::{self, orm::schema::New as Migration};

use super::{
    env::{self, Config},
    errors::{Error, Result},
};

pub struct Server {}

impl pug::app::Server for Server {
    type Config = Config;
    type Error = Error;
    fn launch(&self, cfg: &Self::Config) -> Result<()> {
        pug::rocket::custom(cfg.pug.rocket()?)
            // .mount("/", routes![index])
            .launch();;
        Ok(())
    }
    fn migrations(&self) -> Vec<Migration> {
        vec![pug::i18n::locales::migration(), pug::settings::migration()]
    }
}

pub fn launch() -> Result<()> {
    let version = env::version();
    let app = pug::app::App::new(
        env::NAME,
        &version[..],
        Some(env::AUTHORS),
        Some(env::DESCRIPTION),
        Some(env::BANNER),
        Some(env::HOMEPAGE),
    );
    app.launch(&Server {})
}
