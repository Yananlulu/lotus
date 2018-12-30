use std::net::IpAddr;

use chrono::{NaiveDateTime, Utc};
use diesel::{insert_into, prelude::*};
use ipnetwork::IpNetwork;
use pug::orm::Connection;
use serde_json::Value;

use super::super::super::super::errors::Result;
use super::schema::*;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub form_id: i64,
    pub ip: IpNetwork,
    pub content: Value,
    pub created_at: NaiveDateTime,
}

pub trait Dao {
    fn add(&self, form: &i64, ip: &IpAddr, content: &Value) -> Result<i64>;
    fn by_form(&self, id: &i64) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(&self, form: &i64, ip: &IpAddr, content: &Value) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let ip: IpNetwork = (*ip).into();
        let id = insert_into(survey_responses::dsl::survey_responses)
            .values((
                survey_responses::dsl::form_id.eq(form),
                survey_responses::dsl::ip.eq(&ip),
                survey_responses::dsl::content.eq(content),
                survey_responses::dsl::created_at.eq(&now),
            ))
            .returning(survey_responses::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn by_form(&self, id: &i64) -> Result<Vec<Item>> {
        let items = survey_responses::dsl::survey_responses
            .filter(survey_responses::dsl::form_id.eq(id))
            .order(survey_responses::dsl::created_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}