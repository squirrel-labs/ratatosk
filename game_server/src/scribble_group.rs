use crate::group::{Group, GroupId};
use crate::server::{UserId, GameClient,
                    ClientSender, ClientReceiver,
                    GameServerError};
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

    fn add_client(&mut self, id: UserId, client: GameClient) -> Result<(), GameServerError> {
        debug!("user {} joined the group {}:'{}'", id, self.id, self.name);
        let (sen, rec) = client.split();
        self.senders.lock().unwrap().insert(id, sen);
        let senders_mutex = self.senders.clone();
        let self_uid = id;
        std::thread::spawn(move || Self::broadcast_clients(self_uid, rec, senders_mutex));
        Ok(())
    }
}

impl ScribbleGroup {
    pub fn new(id: GroupId, name: String) -> Self {
        Self { id, name, senders: Arc::new(Mutex::new(HashMap::new())) }
    }

    fn broadcast_clients(self_uid: UserId, mut rec: ClientReceiver, senders_mutex: Arc<Mutex<HashMap<UserId, ClientSender>>>) {
        loop {
            let message = match rec.recv_message() {
                Ok(x) => x,
                _ => break
            };
            //trace!("got message: '{:?}'", message);
            let mut senders = senders_mutex.lock().unwrap();
            for (uid, sender) in senders.iter_mut() {
                if self_uid != *uid {
                    sender.send_message(&message)
                        .unwrap_or_else(|_| debug!("tried to send message to {}, but failed", *uid));
                }
            }
        }
        senders_mutex.lock().unwrap().remove(&self_uid);
        info!("client {} has left", self_uid);
    }
}
