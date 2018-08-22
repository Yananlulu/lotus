use juniper;

use super::super::{errors::Result, orm::PooledConnection as Db};

pub struct CurrentUser {
    pub id: i64,
    pub uid: String,
    pub email: String,
}

pub struct Context {
    pub home: String,
    pub locale: String,
    pub token: Option<String>,
    pub client_ip: String,
    pub db: Db,
}

impl juniper::Context for Context {}

impl Context {
    pub fn client_ip(&self) -> Result<String> {
        Ok("ip".to_string())
    }
}
