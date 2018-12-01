extern crate lotus;
extern crate pug;

use pug::log::*;

fn main() {
    if let Err(e) = lotus::app::launch() {
        error!("{:?}", e);
    }
}
