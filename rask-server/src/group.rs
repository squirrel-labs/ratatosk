use crate::backend_connection::TokenResponse;
use crate::error::ServerError;
use std::convert::TryInto;
use std::sync::mpsc;
use ws::Sender;

pub type GroupId = u32;

pub trait Game {
    fn run(&self, group: &Group);
}

pub struct RaskGame; // {}

impl Game for RaskGame {
    fn run(&self, group: &Group) {}
}

pub struct Group {
    pub clients: Vec<Sender>,
    pub receiver: mpsc::Receiver<Box<[u8]>>,
    sender: mpsc::Sender<Box<[u8]>>,
    id: GroupId,
    group_type: String,
    name: String,
    capacity: u32,
}

impl Group {
    pub fn id(&self) -> GroupId {
        self.id
    }

    pub fn group_type(&self) -> String {
        self.group_type.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn add_client(
        &mut self,
        client: Sender,
    ) -> Result<mpsc::Sender<Box<[u8]>>, crate::error::ServerError> {
        if self.clients.len() >= self.capacity.try_into().unwrap() {
            return Err(ServerError::Group(format!(
                "User limit for {} exceeded",
                self.id
            )));
        }
        self.clients.push(client);
        Ok(self.sender.clone())
    }

    pub fn remove_client(&mut self, client: Sender) {
        if let Some(pos) = self.clients.iter().position(|x| *x == client) {
            self.clients.swap_remove(pos);
        }
    }

    pub fn new(response: TokenResponse) -> Self {
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
        let U = RaskGame;
        U.run(&group);
        group
    }
}
