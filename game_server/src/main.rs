mod group;
mod lobby;

#[macro_use] extern crate log;
use pretty_env_logger;

use group::Group;
use lobby::Lobby;

fn main() {
    pretty_env_logger::init();

    let mut lobby = Lobby::new();
    lobby.add_group(Group::new(0, "Test".to_string()));
    lobby.add_group(Group::new(1, "Very Serious".to_string()));

    for group in lobby.iter() {
        group.run()
    }

    loop {}
}
