pub mod schema;

use chrono::{NaiveDateTime, Utc};
use diesel::{delete, insert_into, prelude::*, update};
use pug::orm::{schema::New as Schema, Connection};

use super::super::super::errors::Result;

use self::schema::*;

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

#[derive(Queryable)]
pub struct Topic {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable)]
pub struct Post {
    pub id: i64,
    pub user_id: i64,
    pub topic_id: i64,
    pub post_id: Option<i64>,
    pub body: String,
    pub media_type: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait TopicDao {
    fn add(
        &self,
        user: &i64,
        title: &String,
        body: &String,
        media_type: &String,
        tags: Vec<i64>,
        cataoried: Vec<i64>,
    ) -> Result<i64>;
    fn get(&self, id: &i64) -> Result<Topic>;
    fn update(
        &self,
        id: &i64,
        title: &String,
        body: &String,
        media_type: &String,
        tags: Vec<i64>,
        cataoried: Vec<i64>,
    ) -> Result<()>;
    fn latest(&self) -> Result<Vec<Topic>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

pub trait PostDao {
    fn add(
        &self,
        user: &i64,
        topic: &i64,
        post: &Option<i64>,
        body: &String,
        media_type: &String,
    ) -> Result<i64>;
    fn get(&self, id: &i64) -> Result<Post>;
    fn update(&self, id: &i64, body: &String, media_type: &String) -> Result<()>;
    fn latest(&self) -> Result<Vec<Post>>;
    fn delete(&self, id: &i64) -> Result<()>;
}

impl TopicDao for Connection {
    fn add(
        &self,
        user: &i64,
        title: &String,
        body: &String,
        media_type: &String,
        tags: Vec<i64>,
        categories: Vec<i64>,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(forum_topics::dsl::forum_topics)
            .values((
                forum_topics::dsl::user_id.eq(user),
                forum_topics::dsl::title.eq(title),
                forum_topics::dsl::body.eq(body),
                forum_topics::dsl::media_type.eq(media_type),
                forum_topics::dsl::updated_at.eq(&now),
            ))
            .returning(forum_topics::dsl::id)
            .get_result(self)?;
        for it in tags {
            insert_into(forum_topics_tags::dsl::forum_topics_tags)
                .values((
                    forum_topics_tags::dsl::tag_id.eq(&it),
                    forum_topics_tags::dsl::topic_id.eq(&id),
                    forum_topics_tags::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        for it in categories {
            insert_into(forum_topics_categories::dsl::forum_topics_categories)
                .values((
                    forum_topics_categories::dsl::category_id.eq(&it),
                    forum_topics_categories::dsl::topic_id.eq(&id),
                    forum_topics_categories::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        Ok(id)
    }
    fn get(&self, id: &i64) -> Result<Topic> {
        let it = forum_topics::dsl::forum_topics
            .filter(forum_topics::dsl::id.eq(id))
            .first::<Topic>(self)?;
        Ok(it)
    }
    fn update(
        &self,
        id: &i64,
        title: &String,
        body: &String,
        media_type: &String,
        tags: Vec<i64>,
        categories: Vec<i64>,
    ) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(id));
        update(it)
            .set((
                forum_topics::dsl::title.eq(title),
                forum_topics::dsl::body.eq(body),
                forum_topics::dsl::media_type.eq(media_type),
                forum_topics::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        delete(
            forum_topics_tags::dsl::forum_topics_tags
                .filter(forum_topics_tags::dsl::topic_id.eq(id)),
        )
        .execute(self)?;
        for it in tags {
            insert_into(forum_topics_tags::dsl::forum_topics_tags)
                .values((
                    forum_topics_tags::dsl::tag_id.eq(&it),
                    forum_topics_tags::dsl::topic_id.eq(id),
                    forum_topics_tags::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }

        delete(
            forum_topics_categories::dsl::forum_topics_categories
                .filter(forum_topics_categories::dsl::topic_id.eq(id)),
        )
        .execute(self)?;
        for it in categories {
            insert_into(forum_topics_categories::dsl::forum_topics_categories)
                .values((
                    forum_topics_categories::dsl::category_id.eq(&it),
                    forum_topics_categories::dsl::topic_id.eq(id),
                    forum_topics_categories::dsl::created_at.eq(&now),
                ))
                .execute(self)?;
        }
        Ok(())
    }
    fn latest(&self) -> Result<Vec<Topic>> {
        let items = forum_topics::dsl::forum_topics
            .order(forum_topics::dsl::updated_at.desc())
            .load::<Topic>(self)?;
        Ok(items)
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(forum_topics::dsl::forum_topics.filter(forum_topics::dsl::id.eq(id)))
            .execute(self)?;
        Ok(())
    }
}

impl PostDao for Connection {
    fn add(
        &self,
        user: &i64,
        topic: &i64,
        post: &Option<i64>,
        body: &String,
        media_type: &String,
    ) -> Result<i64> {
        let now = Utc::now().naive_utc();
        let id = insert_into(forum_posts::dsl::forum_posts)
            .values((
                forum_posts::dsl::user_id.eq(user),
                forum_posts::dsl::topic_id.eq(topic),
                forum_posts::dsl::post_id.eq(post),
                forum_posts::dsl::body.eq(body),
                forum_posts::dsl::media_type.eq(media_type),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .returning(forum_posts::dsl::id)
            .get_result(self)?;
        Ok(id)
    }
    fn get(&self, id: &i64) -> Result<Post> {
        let it = forum_posts::dsl::forum_posts
            .filter(forum_posts::dsl::id.eq(id))
            .first::<Post>(self)?;
        Ok(it)
    }
    fn update(&self, id: &i64, body: &String, media_type: &String) -> Result<()> {
        let now = Utc::now().naive_utc();
        let it = forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(id));
        update(it)
            .set((
                forum_posts::dsl::body.eq(body),
                forum_posts::dsl::media_type.eq(media_type),
                forum_posts::dsl::updated_at.eq(&now),
            ))
            .execute(self)?;

        Ok(())
    }
    fn latest(&self) -> Result<Vec<Post>> {
        let items = forum_posts::dsl::forum_posts
            .order(forum_posts::dsl::updated_at.desc())
            .load::<Post>(self)?;
        Ok(items)
    }
    fn delete(&self, id: &i64) -> Result<()> {
        delete(forum_posts::dsl::forum_posts.filter(forum_posts::dsl::id.eq(id))).execute(self)?;
        Ok(())
    }
}
