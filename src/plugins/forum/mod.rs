pub mod controllers;
pub mod dao;

use rocket::Route;

lazy_static! {
    pub static ref ROUTES: Vec<(&'static str, Vec<Route>)> = {
        let mut items = Vec::new();
        items.push(("/api/forum", routes![controllers::posts::new]));
        items
    };
}
