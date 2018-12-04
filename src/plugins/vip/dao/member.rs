use chrono::{Datelike, NaiveDate, NaiveDateTime, Utc};

#[derive(Queryable)]
pub struct Member {
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Member {
    pub fn age(&self) -> i32 {
        Utc::now().year() - self.birthday.year()
    }
}
