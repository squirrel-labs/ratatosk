mod group;
mod test_group;
mod lobby;
mod gameserver;
mod backend_connection;

#[macro_use] extern crate log;
use pretty_env_logger;

use backend_connection::BackendConnection;

fn main() {
    pretty_env_logger::init();

    let addr = ("0.0.0.0", 5001);
    info!("create game server on {:?}", addr);
    let gameserver = gameserver::GameServer::new(addr);
    gameserver.run().unwrap();

}
