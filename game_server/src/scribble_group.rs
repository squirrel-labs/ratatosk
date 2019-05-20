use super::group::{Group, GroupId};
use super::server::{UserId, GameClient,
                    ClientSender, ClientReceiver};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub struct ScribbleGroup {
    id: GroupId,
    name: String,
    senders: Arc<Mutex<HashMap<UserId, ClientSender>>>
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
        let (sen, mut rec) = client.split();
        self.senders.lock().unwrap().insert(id, sen);
        let senders_mutex = self.senders.clone();
        let self_uid = id;
        std::thread::spawn(move || {
            loop {
                let message = rec.recv_message().unwrap();
                info!("got message: '{:?}'", message);
                let mut senders = senders_mutex.lock().unwrap();
                for (uid, sender) in senders.iter_mut() {
                    if self_uid != *uid {
                        sender.send_message(&message);
                    }
                }
            }
        });
    }
}

impl ScribbleGroup {
    pub fn new(id: GroupId, name: String) -> Self {
        Self { id, name, senders: Arc::new(Mutex::new(HashMap::new())) }
    }
}
