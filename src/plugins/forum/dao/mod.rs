pub mod post;
pub mod schema;
pub mod topic;

use pug::orm::schema::New as Schema;

pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

pub fn migrations<'a>() -> Schema<'a> {
    Schema {
        version: "20181209131944999124229",
        name: "create-forum",
        up: UP,
        down: DOWN,
    }
}
