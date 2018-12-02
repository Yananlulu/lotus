extern crate lotus;
extern crate pug;

fn main() {
    if let Err(e) = lotus::app::launch() {
        panic!("{:?}", e);
    }
}
