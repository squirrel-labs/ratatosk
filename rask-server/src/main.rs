mod backend_connection;
mod error;
mod group;

mod game_logger;

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
use clap::App;
pub use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    game_logger::init_logger();

    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let addr = matches.value_of("address").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("5001");

    let addr = (addr, port);
    info!("create game server on {:?}", addr);
    Ok(())
}
