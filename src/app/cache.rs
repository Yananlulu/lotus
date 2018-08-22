use super::super::errors::Result;

pub fn list() -> Result<()> {
    let cfg = super::parse_config()?;
    let ch = cfg.cache.open()?;
    let items = ch.keys()?;
    println!("{:64} {}", "KEY", "TTL");
    for (key, ttl) in items {
        println!("{:64} {}", key, ttl);
    }
    Ok(())
}

pub fn clear() -> Result<()> {
    let cfg = super::parse_config()?;
    let ch = cfg.cache.open()?;
    let cnt = ch.clear()?;
    info!("remove {} items from cache", cnt);
    Ok(())
}
