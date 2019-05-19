mod group;
mod test_group;
mod lobby;
mod gameserver;
mod backend_connection;

mod game_logger;

#[macro_use] extern crate log;

use backend_connection::BackendConnection;

fn main() {
    game_logger::init_logger();

    let addr = ("127.0.0.1", 5001);
    info!("create game server on {:?}", addr);
    let gameserver = gameserver::GameServer::new(addr);
    gameserver.run().unwrap();

}
