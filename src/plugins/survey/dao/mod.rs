pub mod schema;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use pug::orm::{schema::New as Schema, Connection, ID};

use super::super::super::errors::{Error, Result};

pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

pub fn migrations<'a>() -> Schema<'a> {
    Schema {
        version: "20181209215353741444028",
        name: "create-survey",
        up: UP,
        down: DOWN,
    }
}
