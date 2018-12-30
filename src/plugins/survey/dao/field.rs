use chrono::NaiveDateTime;
use pug::orm::Connection;
use serde_json::Value;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub form_id: i64,
    pub key: String,
    pub title: String,
    pub description: Option<String>,
    pub required: bool,
    pub type_: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub enum Type {
    SELECT,
    RADIO(),
}

pub trait Dao {}

impl Dao for Connection {}
