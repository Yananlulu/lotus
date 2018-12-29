pub mod schema;

use std::fmt;
use std::str::FromStr;

use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use pug::orm::{schema::New as Schema, Connection};
use serde_json::{from_value, to_value, Value};

use super::super::super::errors::{Error, Result};

use self::schema::*;

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
    pub contact: Value,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Item {
    pub fn age(&self) -> i32 {
        Utc::now().year() - self.birthday.year()
    }
    pub fn contact(self) -> Result<Contact> {
        let it = from_value(self.contact)?;
        Ok(it)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Contact {
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub line: Option<String>,
    pub wechat: Option<String>,
    pub skype: Option<String>,
    pub weibo: Option<String>,
    pub facebook: Option<String>,
    pub twitter: Option<String>,
}

#[derive(Insertable)]
#[table_name = "vip_members"]
pub struct New<'a> {
    pub nick_name: &'a str,
    pub real_name: &'a str,
    pub gender: &'a str,
    pub birthday: &'a NaiveDate,
    pub contact: &'a Value,
    pub updated_at: &'a NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        nick_name: &String,
        real_name: &String,
        gender: &Gender,
        birthday: &NaiveDate,
        contact: &Contact,
    ) -> Result<i64>;
    fn get(&self, id: &i64) -> Result<Item>;
    fn update(
        &self,
        id: &i64,
        real_name: &String,
        gender: &Gender,
        birthday: &NaiveDate,
        contact: &Contact,
    ) -> Result<()>;
    fn list(&self) -> Result<Vec<Item>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl Dao for Connection {
    fn add(
        &self,
        nick_name: &String,
        real_name: &String,
        gender: &Gender,
        birthday: &NaiveDate,
        contact: &Contact,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(vip_members::dsl::vip_members)
            .values((
                vip_members::dsl::nick_name.eq(nick_name),
                vip_members::dsl::real_name.eq(real_name),
                vip_members::dsl::birthday.eq(&birthday),
                vip_members::dsl::gender.eq(&gender.to_string()),
                vip_members::dsl::contact.eq(&to_value(contact)?),
                vip_members::dsl::updated_at.eq(&now),
            ))
            .returning(vip_members::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn get(&self, id: &i64) -> Result<Item> {
        let it = vip_members::dsl::vip_members
            .filter(vip_members::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn update(
        &self,
        id: &i64,
        real_name: &String,
        gender: &Gender,
        birthday: &NaiveDate,
        contact: &Contact,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = vip_members::dsl::vip_members.filter(vip_members::dsl::id.eq(id));
        update(it)
            .set((
                vip_members::dsl::real_name.eq(real_name),
                vip_members::dsl::birthday.eq(&birthday),
                vip_members::dsl::gender.eq(&gender.to_string()),
                vip_members::dsl::contact.eq(&to_value(contact)?),
                vip_members::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        Ok(())
    }
    fn list(&self) -> Result<Vec<Item>> {
        let items = vip_members::dsl::vip_members
            .order(vip_members::dsl::nick_name.asc())
            .load::<Item>(self)?;
        Ok(items)
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(vip_members::dsl::vip_members.filter(vip_members::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
