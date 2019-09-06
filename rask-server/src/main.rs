mod backend_connection;
mod error;
mod group;
mod lobby;
mod server;

mod game_logger;

#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
use clap::App;
pub use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    game_logger::init_logger();

    // load args
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    // extract values from args
    let addr = matches.value_of("address").unwrap_or("127.0.0.1");
    let port = matches.value_of("port").unwrap_or("5001");

    // start server
    info!("create game server on {:?}", addr);
    let ws_server = server::Server::run(addr, port);
    ws_server.unwrap().join();
    //TODO fix unwrap
    Ok(())
}
