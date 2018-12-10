use pug::{nut, orm::schema::New as Migration};
use rocket;

use super::{
    env::{self, Config},
    errors::{Error, Result},
    plugins,
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
            pug::i18n::db::migrations(),
            pug::settings::migrations(),
            pug::nut::auth::migrations(),
            pug::nut::site::migrations(),
            plugins::forum::dao::migrations(),
            plugins::vip::dao::migrations(),
            plugins::survey::dao::migrations(),
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
