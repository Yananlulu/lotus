use std::sync::Arc;

use gotham;

use super::super::{env::Config, errors::Result, router};

pub fn server(cfg: Arc<Config>) -> Result<()> {
    let addr = format!("127.0.0.1:{}", cfg.http.port);
    info!("Listening for requests at http://{}", addr);
    gotham::start(addr, router::router());
    Ok(())
}

pub fn routes() -> Result<()> {
    println!("{}\t{}", "METHOD", "PATH");
    Ok(())
}
