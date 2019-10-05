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
    pub sender: mpsc::Sender<Message>,
    id: GroupId,
    group_type: String,
    name: String,
    capacity: u32,
    game_thread: JoinHandle<()>,
}

pub struct SendGroup {
    pub receiver: mpsc::Receiver<Message>,
    pub id: GroupId,
    pub group_type: String,
    pub name: String,
    pub capacity: u32,
}

pub enum Message {
    // TODO: flatten tuple
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
        self.game_thread.thread().unpark();
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

    pub fn new(response: TokenResponse) -> Result<Self, ServerError> {
        let (sender, receiver) = mpsc::channel();
        let (id, name, group_type) = (response.group_id, response.group_name, response.group_type);
        info!(
            "Creating Group{} ({}) with game {}",
            id, name, group_type
        );

        let send_group = SendGroup {
            receiver,
            id, name: name.clone(), group_type: group_type.clone(),
            capacity: response.user_max
        };

        let game = match group_type.as_str() {
            "rask" => RaskGame::new(send_group),
            name => Err(ServerError::GroupCreation(format!(
                    "The game type {} is not implemented",
                    name
                )))?
        };

        Ok(Self {
            clients: Vec::new(),
            sender, id: id, group_type, name,
            capacity: response.user_max,
            game_thread: game.run()?,
        })
    }
}
