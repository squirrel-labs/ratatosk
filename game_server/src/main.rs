mod group;
mod test_group;
mod lobby;
mod backend_connection;

#[macro_use] extern crate log;
use pretty_env_logger;

use test_group::TestGroup;
use lobby::Lobby;
use backend_connection::BackendConnection;

fn main() {
    pretty_env_logger::init();

    let mut lobby = Lobby::new();

    for group in lobby.iter() {
        group.run()
    }

    let mut backend = BackendConnection::new("http://129.13.215.68:5000");
    loop {
        backend.request("/scribble").unwrap();
        println!("{:?}", backend.get_response());
    }
}
