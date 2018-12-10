pub mod schema;

use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use pug::orm::{schema::New as Schema, Connection, ID};

use super::super::super::errors::{Error, Result};

pub const UP: &'static str = include_str!("up.sql");
pub const DOWN: &'static str = include_str!("down.sql");

pub fn migrations<'a>() -> Schema<'a> {
    Schema {
        version: "20181209215002787834240",
        name: "create-vip",
        up: UP,
        down: DOWN,
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Gender {
    Male,
    Female,
    Others,
}

impl fmt::Display for Gender {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Gender::Male => fmt.write_str("male"),
            Gender::Female => fmt.write_str("female"),
            Gender::Others => fmt.write_str("others"),
        }
    }
}

impl FromStr for Gender {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "male" => Ok(Gender::Male),
            "female" => Ok(Gender::Female),
            "others" => Ok(Gender::Others),
            v => Err(format!("unknown gender {}", v).into()),
        }
    }
}

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub nick_name: String,
    pub real_name: String,
    pub gender: String,
    pub birthday: NaiveDate,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn age(&self) -> i32 {
        Utc::now().year() - self.birthday.year()
    }
}

// #[derive(Insertable)]
// #[table_name = "vip_members"]
// pub struct New<'a> {
//     pub real_name: &'a str,
//     pub nick_name: &'a str,
//     pub email: &'a str,
//     pub password: Option<&'a [u8]>,
//     pub uid: &'a str,
//     pub provider_type: &'a str,
//     pub provider_id: &'a str,
//     pub logo: &'a str,
//     pub updated_at: &'a NaiveDateTime,
// }
