mod backend_connection;
mod group;
mod lobby;
mod scribble_group;
mod server;
mod webhogg_group;

mod game_logger;

#[macro_use]
extern crate log;

fn main() {
    game_logger::init_logger();

    let addr = ("0.0.0.0", 5001);
    info!("create game server on {:?}", addr);
    let mut gameserver = server::GameServer::new(addr);
    gameserver.run().unwrap();
}
