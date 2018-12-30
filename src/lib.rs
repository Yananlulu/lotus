#![feature(proc_macro_hygiene, decl_macro)]
#![recursion_limit = "1024"]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;
#[macro_use]
extern crate rocket_contrib;

extern crate chrono;
extern crate ipnetwork;
extern crate pug;
extern crate serde_json;
extern crate validator;

pub mod app;
pub mod env;
pub mod errors;
pub mod plugins;
