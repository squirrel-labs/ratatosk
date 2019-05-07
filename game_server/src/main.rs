mod lobby;
mod group;

#[macro_use] extern crate log;
use pretty_env_logger;

fn main() {
    pretty_env_logger::init();

    trace!("test info");
}
