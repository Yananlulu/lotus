use chrono::NaiveDateTime;
use ipnetwork::IpNetwork;
use pug::orm::Connection;
use serde_json::Value;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub form_id: i64,
    pub ip: IpNetwork,
    pub body: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {}

impl Dao for Connection {}
