use crate::backend_connection::TokenResponse;
use crate::error::ServerError;
use crate::games::{Game, RaskGame};
use std::convert::TryInto;
use std::sync::mpsc;
use ws::Sender;

pub type GroupId = u32;

pub struct Group {
    pub clients: Vec<Sender>,
    pub receiver: mpsc::Receiver<Message>,
    sender: mpsc::Sender<Message>,
    id: GroupId,
    group_type: String,
    name: String,
    capacity: u32,
}

pub type Message = (String, Content);

pub enum Content {
    data(Box<Vec<u8>>),
    park,
    kill,
}

impl Group {
    pub fn id(&self) -> GroupId {
        self.id
    }

    pub fn group_type(&self) -> &str {
        self.group_type.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn add_client(&mut self, client: Sender) -> Result<mpsc::Sender<Message>, ServerError> {
        if self.clients.len() >= self.capacity.try_into().unwrap() {
            return Err(ServerError::Group(format!(
                "User limit for {} exceeded",
                self.id
            )));
        }
        self.clients.push(client);
        Ok(self.sender.clone())
    }

    pub fn remove_client(&mut self, client: &Sender) {
        if let Some(pos) = self.clients.iter().position(|x| *x == *client) {
            self.clients.swap_remove(pos);
        }
    }

    pub fn new(response: TokenResponse) -> Result<Self, ServerError> {
        let (sender, receiver) = mpsc::channel();
        let group = Self {
            clients: Vec::new(),
            sender,
            receiver,
            id: response.group_id,
            group_type: response.group_type,
            name: response.group_name,
            capacity: response.user_max,
        };
        info!("Creating Group{} ({}) with game {}", group.id(), group.name(), group.group_type());
        let game = match group.group_type() {
            "rask" => RaskGame::new(&group),
            name => {
                return Err(ServerError::GroupCreation(format!(
                    "The game type {} is not implemented",
                    name
                )))
            }
        };
        game.run();
        Ok(group)
    }
}
