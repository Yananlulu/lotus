use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::super::{errors::Result, orm::schema::schema_migrations, rfc::RFC822};

pub fn versions() -> Result<()> {
    let cfg = super::parse_config()?;
    let db = cfg.database.open()?;
    let db = db.get()?;
    println!("{:16} {}", "VERSION", "RUN ON");
    for (v, r) in schema_migrations::dsl::schema_migrations
        .select((
            schema_migrations::dsl::version,
            schema_migrations::dsl::created_at,
        ))
        .load::<(String, NaiveDateTime)>(&db)?
    {
        println!("{:16} {}", v, r.to_rfc822());
    }

    Ok(())
}
