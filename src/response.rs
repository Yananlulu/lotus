use std::panic::RefUnwindSafe;

use gotham::{
    handler::{Handler, IntoResponse},
    http::response::create_response,
    state::State,
};
use hyper::{Response, StatusCode};
use mime;
use serde::ser::Serialize;
use serde_xml_rs;

use super::errors::{Error, Result};

pub fn error(s: &State, e: Error) -> Response {
    create_response(
        s,
        StatusCode::InternalServerError,
        Some((e.description().as_bytes().to_vec(), mime::TEXT_PLAIN_UTF_8)),
    )
}

pub fn xml<V, H>(state: State, handler: H) -> (State, Response)
where
    V: Serialize,
    H: Fn(&State) -> Result<V>,
{
    let res = match handler(&state) {
        Ok(v) => {
            let mut buf = Vec::new();
            match serde_xml_rs::serialize(&v, &mut buf) {
                Ok(_) => create_response(&state, StatusCode::Ok, Some((buf, mime::TEXT_XML))),
                Err(e) => error(&state, e.into()),
            }
        }
        Err(e) => error(&state, e),
    };
    (state, res)
}

// impl<T: Serialize> IntoResponse for Result<T> {
//     fn into_response(self, state: &State) -> Response {
//         create_response(
//             state,
//             StatusCode::Ok,
//             Some(("aaa".into_bytes(), mime::APPLICATION_JSON)),
//         )
//     }
// }
