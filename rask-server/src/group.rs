use ws::Sender;
use std::convert::TryInto;
use std::sync::mpsc;
use crate::backend_connection::TokenResponse;
use crate::error::ServerError;

pub type GroupId = u32;

pub trait Game {
    fn run(group: &Group);
}

pub struct Group<'a> {
    pub clients: Vec<Sender>,
    pub receiver: mpsc::Receiver<&'a [u8]>,
    sender: mpsc::Sender<&'a [u8]>,
    id: GroupId,
    group_type: String,
    name: String,
    capacity: u32,
}

impl Group<'_> {
    pub fn id(&self) -> GroupId {
        self.id
    }

    pub fn group_type(&self) -> String {
        self.group_type   
    }

    pub fn name(&self) -> String {
        self.name   
    }

    pub fn add_client(&mut self, client: Sender) -> Result<mpsc::Sender<&[u8]>, crate::error::ServerError> {
        if self.clients.len() >= self.capacity.try_into().unwrap() {
            return Err(ServerError::Group(format!("User limit for {} exceeded", self.id)));
        }
        self.clients.push(client);
        Ok(self.sender.clone())
    }

    pub fn remove_client(&mut self, client: Sender) {
        if let Some(pos) = self.clients.iter().position(|x| *x == client) {
            self.clients.swap_remove(pos);
        }
    }

    pub fn new<U: Game>(response: TokenResponse, game: U) -> Self {
        let (sender, receiver) = mpsc::channel();
        let group = Self {
            clients:  Vec::new(),
            sender,
            receiver,
            id: response.group_id,
            group_type: response.group_type,
            name: response.group_name,
            capacity: response.user_max,
        };
        U::run(&group);
        group
    }
}


