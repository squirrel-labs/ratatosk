use std::convert::TryInto;
use std::sync::mpsc;
use std::thread::JoinHandle;

use crate::backend_connection::TokenResponse;
use crate::error::ServerError;
use crate::games;
use crate::games::{Game, RaskGame};
use log::info;
use ws::Sender;

pub type GroupId = u32;

#[derive(Debug)]
/// capacity is never allowed to be above usize::MAX
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

#[derive(Debug)]
pub enum Message {
    // TODO: flatten tuple
    Data((String, Vec<u8>)),
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
        info!("dropping group {:?}", &self);
        let _ = self.sender.send(Message::Kill);
    }
}

impl Group {
    pub fn id(&self) -> GroupId {
        self.id
    }

    #[allow(dead_code)]
    pub fn group_type(&self) -> &str {
        &self.group_type
    }

    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[allow(dead_code)]
    pub fn park(self) -> Result<(), ServerError> {
        Ok(self.sender.send(Message::Park)?)
    }

    #[allow(dead_code)]
    pub fn unpark(&mut self) {
        self.game_thread.thread().unpark();
    }

    pub fn add_client(&mut self, client: Sender) -> Result<mpsc::Sender<Message>, ServerError> {
        if self.clients.len() >= self.capacity as usize {
            Err(ServerError::Group(format!(
                "User limit for {} exceeded",
                self.id
            )))
        } else {
            self.clients.push(client.clone());
            self.sender
                .send(Message::Add(games::User::new("None".to_owned(), client)))
                .map_err(Into::into)
                .map(|()| self.sender.clone())
        }
    }

    pub fn remove_client(&mut self, client: &Sender) -> Result<(), ServerError> {
        if let Some(pos) = self.clients.iter().position(|x| *x == *client) {
            self.clients.swap_remove(pos);
        }
        self.sender
            .send(Message::Remove(client.clone()))
            .map_err(Into::into)
    }

    pub fn new(response: TokenResponse) -> Result<Self, ServerError> {
        let (sender, receiver) = mpsc::channel();
        let (id, name, group_type) = (response.group_id, response.group_name, response.group_type);
        let capacity = response.user_max.try_into().unwrap_or(std::usize::MAX) as u32;
        info!("Creating Group{} ({}) with game {}", id, name, group_type);

        let send_group = SendGroup {
            receiver,
            id,
            name: name.clone(),
            group_type: group_type.clone(),
            capacity,
        };

        let game = match group_type.as_str() {
            "rask" => RaskGame::new(send_group),
            name => {
                return Err(ServerError::GroupCreation(format!(
                    "The game type {} is not implemented",
                    name
                )))
            }
        };

        Ok(Self {
            clients: Vec::new(),
            sender,
            id,
            group_type,
            name,
            capacity,
            game_thread: game.run()?,
        })
    }
}
