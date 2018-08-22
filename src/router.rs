// use std::io::Cursor;
// use std::ops::Deref;
// use std::path::{Path, PathBuf};
// use std::sync::Arc;
//

//
// use super::{
//     env,
//     errors::{Error, Result},
//     orm::PooledConnection as Db,
//     plugins::{forum, nut},
// };

// impl<'r> Responder<'r> for Error {
//     fn respond_to(self, _: &Request) -> StdResult<Response<'r>, Status> {
//         Ok(Response::build()
//             .header(ContentType::Plain)
//             .status(Status::InternalServerError)
//             .sized_body(Cursor::new(self.description().to_string()))
//             .finalize())
//     }
// }
//
// pub fn routes() -> Vec<(&'static str, Vec<Route>)> {
//     let mut items = Vec::new();
//     items.extend_from_slice(&nut::routes());
//     items.extend_from_slice(&forum::routes());
//     items.push((
//         "/",
//         routes![
//             third,
//             upload,
//             global,
//             assets,
//             robots_txt,
//             sitemap_xml_gz,
//             rss_atom
//         ],
//     ));
//     items
// }
//
// pub fn catchers() -> Vec<Catcher> {
//     catchers![not_found, forbidden, internal_server_error]
// }
//

//
// #[catch(404)]
// fn not_found() -> &'static str {
//     Status::NotFound.reason
// }
//
// #[catch(403)]
// fn forbidden() -> &'static str {
//     Status::Forbidden.reason
// }
//
// #[catch(500)]
// fn internal_server_error() -> &'static str {
//     Status::InternalServerError.reason
// }
