use super::group::{Group, GroupId};
use super::server::{UserId, GameClient};
use std::collections::HashMap;

pub struct ScribbleGroup {
    id: GroupId,
    name: String,
    clients: HashMap<UserId, GameClient>
}

impl Group for ScribbleGroup {
    fn id(&self) -> GroupId {
        self.id
    }

    fn group_type(&self) -> String {
        "scribble".to_string()
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn run(&mut self) {
        info!("a new group {}:'{}' runs now", self.id, self.name);
    }

    fn add_client(&mut self, id: UserId, client: GameClient) {
        debug!("user {} joined the group {}:'{}'", id, self.id, self.name);
        self.clients.insert(id, client);
    }

    fn get_client(&self, client_id: UserId) -> Option<&GameClient> {
        self.clients.get(&client_id)
    }
}

impl ScribbleGroup {
    pub fn new(id: GroupId, name: String) -> Self {
        Self { id, name, clients: HashMap::new() }
    }
}
