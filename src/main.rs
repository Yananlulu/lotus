// extern crate log4rs;
extern crate lotus;

#[macro_use]
extern crate log;

fn main() {
    let run = || {
        // log4rs::init_file("log4rs.yml", Default::default())?;
        lotus::app::main()
    };
    if let Err(e) = run() {
        error!("{:?}", e);
    }
}
