use gotham::{http::response::create_response, state::State};
use hyper::{Response, StatusCode};
use mime;

pub fn index(state: State) -> (State, Response) {
    let res = create_response(
        &state,
        StatusCode::Ok,
        Some((String::from("Hello World!").into_bytes(), mime::TEXT_PLAIN)),
    );

    (state, res)
}
