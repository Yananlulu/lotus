use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use pug::{nut::MediaType, orm::Connection};

use super::super::super::super::errors::Result;
use super::schema::*;

#[derive(Queryable)]
pub struct Item {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub nbf: NaiveDate,
    pub exp: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub trait Dao {
    fn add(
        &self,
        user: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<i64>;

    fn update(
        &self,
        id: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()>;
    fn get(&self, id: &i64) -> Result<Item>;
    fn delete(&self, id: &i64) -> Result<()>;
    fn latest(&self) -> Result<Vec<Item>>;
}

impl Dao for Connection {
    fn add(
        &self,
        user: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(survey_forms::dsl::survey_forms)
            .values((
                survey_forms::dsl::user_id.eq(user),
                survey_forms::dsl::title.eq(title),
                survey_forms::dsl::body.eq(body),
                survey_forms::dsl::media_type.eq(&media_type.to_string()),
                survey_forms::dsl::nbf.eq(nbf),
                survey_forms::dsl::exp.eq(exp),
                survey_forms::dsl::updated_at.eq(&now),
            ))
            .returning(survey_forms::dsl::id)
            .get_result(self)?;
        Ok(id)
    }

    fn update(
        &self,
        id: &i64,
        title: &String,
        body: &String,
        media_type: &MediaType,
        nbf: &NaiveDate,
        exp: &NaiveDate,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = survey_forms::dsl::survey_forms.filter(survey_forms::dsl::id.eq(id));
        update(it)
            .set((
                survey_forms::dsl::title.eq(title),
                survey_forms::dsl::body.eq(body),
                survey_forms::dsl::media_type.eq(&media_type.to_string()),
                survey_forms::dsl::nbf.eq(nbf),
                survey_forms::dsl::exp.eq(exp),
                survey_forms::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;
        Ok(())
    }

    fn get(&self, id: &i64) -> Result<Item> {
        let it = survey_forms::dsl::survey_forms
            .filter(survey_forms::dsl::id.eq(id))
            .first::<Item>(self)?;
        Ok(it)
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(survey_forms::dsl::survey_forms.filter(survey_forms::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }

    fn latest(&self) -> Result<Vec<Item>> {
        let items = survey_forms::dsl::survey_forms
            .order(survey_forms::dsl::updated_at.desc())
            .load::<Item>(self)?;
        Ok(items)
    }
}
