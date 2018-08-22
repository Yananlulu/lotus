use std::sync::Arc;

use rocket;
use rocket_contrib::Template;

use super::super::{env::Config, errors::Result, plugins, router};

pub fn server(cfg: Arc<Config>) -> Result<()> {
    let mut app = rocket::custom(cfg.rocket()?, false);
    for (k, v) in plugins::routes() {
        app = app.mount(k, v);
    }
    app = app
        // .manage(ctx.db.clone())
        // .manage(Arc::clone(&ctx))
        // .manage(graphql::schema::Schema::new(
        //     graphql::query::Query,
        //     graphql::mutation::Mutation,
        // ))
        .attach(Template::fairing())
        .catch(plugins::catchers());
    Err(app.launch().into())
}

pub fn routes() -> Result<()> {
    let cfg = rocket::config::Config::build(rocket::config::Environment::Production)
        .log_level(rocket::config::LoggingLevel::Critical)
        .finalize()?;
    let mut app = rocket::custom(cfg, false);
    for (k, v) in plugins::routes() {
        app = app.mount(k, v);
    }
    println!("{}\t{}\t{}", "METHOD", "RANK", "PATH");
    let items = app.routes();
    for it in items {
        println!("{}\t{}\t{}", it.method, it.rank, it.uri);
    }
    Ok(())
}
