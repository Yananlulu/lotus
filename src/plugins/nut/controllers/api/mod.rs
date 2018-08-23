pub mod users;

use std::net::SocketAddr;
use std::ops::Add;
use std::ops::Deref;

use chrono::{Duration, Utc};
use diesel::Connection;
use rocket_contrib::{Json, Value};
use validator::Validate;

use super::super::super::super::{
    errors::{Error, Result},
    orm::PooledConnection as Db,
    request::Locale,
    utils,
};
use super::super::{dao, models::Role};

#[post("/install", format = "application/json", data = "<form>")]
fn install(
    db: Db,
    lng: Locale,
    remote: SocketAddr,
    form: Json<users::SignUp>,
) -> Result<Json<Value>> {
    form.validate()?;

    let Locale(lng) = lng;
    let ip = format!("{}", remote.ip());
    let db = db.deref();

    db.transaction::<_, Error, _>(|| {
        if dao::user::count(db)? > 0 {
            return Err(t!(db, &lng, "nut.errors.database-not-empty").into());
        }
        let (user, _) = dao::user::add_by_email(db, &form.name, &form.email, &form.password)?;
        l!(db, &user, &ip, &lng, "nut.logs.user.sign-up")?;
        dao::user::confirm(db, &user)?;
        l!(db, &user, &ip, &lng, "nut.logs.user.confirm")?;
        let now = Utc::now().naive_utc();
        let nbf = now.date();
        let exp = now.add(Duration::weeks(1 << 12)).date();
        for it in vec![Role::Admin, Role::Root] {
            dao::policy::apply(db, &user, &it, &None, &nbf, &exp)?;
            l!(
                db,
                &user,
                &ip,
                &lng,
                "nut.logs.role.apply",
                &Some(json!({
                            "name":format!("{}", it),
                            "type": None::<String>,
                            "id": None::<i64>,
                            "exp": exp.format(utils::DATE_FORMAT).to_string(),
                            "nbf": nbf.format(utils::DATE_FORMAT).to_string(),
                        }))
            )?;
        }
        Ok(())
    })?;
    Ok(Json(json!({})))
}
