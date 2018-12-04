use pug::{nut, orm::schema::New as Migration};
use rocket;

use super::{
    env::{self, Config},
    errors::{Error, Result},
};

pub struct Server {}

impl pug::app::Server for Server {
    type Config = Config;
    type Error = Error;
    fn launch(&self, cfg: &Self::Config) -> Result<()> {
        info!("start background jobs thread");
        let mut routes = Vec::new();
        routes.extend_from_slice(&nut::ROUTES);

        let mut app = rocket::custom(cfg.pug.rocket()?)
            .attach(pug::orm::Database::fairing())
            .attach(pug::redis::Redis::fairing());
        for (k, v) in routes {
            app = app.mount(&k, v);
        }

        let err = app.register(nut::catchers::catchers()).launch();
        Err(err.into())
    }
    fn migrations(&self) -> Vec<Migration> {
        vec![
            pug::i18n::db::migration(),
            pug::settings::migration(),
            pug::nut::auth::migration(),
            pug::nut::site::migration(),
        ]
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
    )?;
    app.launch(&Server {})
}
