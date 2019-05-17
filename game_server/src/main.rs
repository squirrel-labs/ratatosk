mod group;
mod test_group;
mod lobby;
mod gameserver;
mod backend_connection;

#[macro_use] extern crate log;
use pretty_env_logger;

use lobby::Lobby;
use backend_connection::BackendConnection;

fn main() {
    pretty_env_logger::init();

    info!("create lobby");
    let mut lobby = Lobby::new();
    let addr = ("127.0.0.1", 5001);
    info!("create game server on {:?}", addr);
    let gameserver = gameserver::GameServer::new(addr);

    for group in lobby.iter() {
        group.run()
    }

    let mut backend = BackendConnection::new("https://kobert.dev");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));

        //backend.request("/api/lobby/tokens/1230").unwrap();
        //println!("{:?}", backend.get_response());
    }
}
