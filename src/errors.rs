use std::io::Cursor;
use std::result::Result as StdResult;

use rocket::{
    http::{ContentType, Status},
    response::Responder,
    Request, Response,
};

error_chain! {
    foreign_links {
        Pug(pug::errors::Error);
        R2d2(r2d2::Error);
        Validation(validator::ValidationErrors);
        RocketLaunch(rocket::error::LaunchError);
        Diesel(diesel::result::Error);
        SerdeJson(serde_json::Error);
    }
}

impl<'r> Responder<'r> for Error {
    fn respond_to(self, _: &Request) -> StdResult<Response<'r>, Status> {
        error!("{:?}", self);
        Ok(Response::build()
            .header(ContentType::Plain)
            .status(Status::InternalServerError)
            .sized_body(Cursor::new(self.description().to_string()))
            .finalize())
    }
}
