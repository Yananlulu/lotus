use rocket_contrib::json::{Json, JsonValue};
use validator::Validate;

use super::super::super::super::errors::Result;

#[derive(Debug, Validate, Deserialize)]
pub struct Form {
    pub id: Option<i64>,
    #[validate(email)]
    pub title: String,
    #[validate(length(min = "1"))]
    pub body: String,
}

#[post("/posts", format = "json", data = "<form>")]
pub fn new(form: Json<Form>) -> Result<JsonValue> {
    form.validate()?;
    Ok(json!({}))
}
