use crate::backend_connection::TokenResponse;
use crate::error::ServerError;
use crate::games;
use crate::games::{Game, RaskGame};
use std::convert::TryInto;
use std::sync::mpsc;
use std::thread::JoinHandle;
use ws::Sender;

pub type GroupId = u32;

pub struct Group {
    pub clients: Vec<Sender>,
    sender: mpsc::Sender<Message>,
    id: GroupId,
    group_type: String,
    name: String,
    capacity: u32,
    game: Option<JoinHandle<()>>,
}

pub struct SendGroup {
    pub clients: Vec<Sender>,
    pub receiver: mpsc::Receiver<Message>,
    pub id: GroupId,
    pub group_type: String,
    pub name: String,
    pub capacity: u32,
}

pub enum Message {
    Data((String, Box<Vec<u8>>)),
    Park,
    Kill,
    Add(games::User),
    Remove(Sender),
}

impl Message {
    pub fn is_data(&self) -> bool {
        match self {
            Message::Data(_) => true,
            _ => false,
        }
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        self.sender.send(Message::Kill).unwrap();
        if let Some(game) = self.game.take() {
            let _ = game.join();
            info!("dropping group");
        }
    }
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

    pub fn park(self) -> Result<(), ServerError> {
        Ok(self.sender.send(Message::Park)?)
    }

    pub fn unpark(&mut self) {
        if let Some(game) = self.game.take() {
            game.thread().unpark();
            self.game = Some(game);
        }
    }

    pub fn add_client(&mut self, client: Sender) -> Result<mpsc::Sender<Message>, ServerError> {
        if self.clients.len() >= self.capacity.try_into().unwrap() {
            return Err(ServerError::Group(format!(
                "User limit for {} exceeded",
                self.id
            )));
        }
        self.clients.push(client.clone());
        let _ = self
            .sender
            .send(Message::Add(games::User::new("None".to_owned(), client)))?;
        Ok(self.sender.clone())
    }

    pub fn remove_client(&mut self, client: &Sender) -> Result<(), ServerError> {
        if let Some(pos) = self.clients.iter().position(|x| *x == *client) {
            self.clients.swap_remove(pos);
        }
        Ok(self.sender.send(Message::Remove(client.clone()))?)
    }

    fn to_send_group(&self, receiver: mpsc::Receiver<Message>) -> SendGroup {
        SendGroup {
            clients: self.clients.clone(),
            receiver,
            id: self.id,
            group_type: self.group_type.to_owned(),
            name: self.name.to_owned(),
            capacity: self.capacity.to_owned(),
        }
    }

    pub fn new(response: TokenResponse) -> Result<Self, ServerError> {
        let (sender, receiver) = mpsc::channel();
        let mut group = Self {
            clients: Vec::new(),
            sender,
            id: response.group_id,
            group_type: response.group_type,
            name: response.group_name,
            capacity: response.user_max,
            game: None,
        };
        info!(
            "Creating Group{} ({}) with game {}",
            group.id(),
            group.name(),
            group.group_type()
        );

        let game = match group.group_type() {
            "rask" => RaskGame::new(group.to_send_group(receiver)),
            name => {
                return Err(ServerError::GroupCreation(format!(
                    "The game type {} is not implemented",
                    name
                )))
            }
        };
        match game.run() {
            Ok(handle) => group.game = Some(handle),
            Err(e) => {
                error!("Os failed to spwan thread. {}", e);
                return Err(e);
            }
        }
        Ok(group)
    }
}
