use crate::error::ServerError;
use crate::group::{Message, SendGroup};
use std::thread;
use std::thread::JoinHandle;

pub trait Game {
    fn run(self) -> Result<JoinHandle<()>, ServerError>;
}

pub struct RaskGame {
    game: (),
    group: SendGroup,
    users: Vec<User>,
    will_to_live: bool,
}

#[derive(Clone)]
pub struct User {
    name: String,
    sender: ws::Sender,
}

impl User {
    pub fn new(name: String, sender: ws::Sender) -> Self {
        User { name, sender }
    }
}

impl Game for RaskGame {
    fn run(self) -> Result<JoinHandle<()>, ServerError> {
        thread::Builder::new()
            .name(format!("group{}", self.group.id))
            .spawn(move || self.game_loop())
            .map_err(|e| ServerError::GameCreation(e))
    }
}

impl RaskGame {
    pub fn new(group: SendGroup) -> Self {
        Self {
            game: (),
            group,
            users: Vec::new(),
            will_to_live: true,
        }
    }

    fn game_loop(mut self) {
        let _messages = self.get_messages();
        while self.will_to_live {
            //game.handle_events(messages);
            //game.tick();
            let _ = self.game == self.game;
            //let b = game.get_broadcast()
            //self.users.iter().foreach(|u| u.sender.send(b));
            let _messages = self.get_messages();
        }
    }

    fn get_messages(&mut self) -> Vec<Message> {
        let (mut data, control): (Vec<Message>, Vec<Message>) =
            self.group.receiver.try_iter().partition(|x| x.is_data());
        control.iter().for_each(|x| match x {
            Message::Park => {
                data = Vec::new();
                thread::park();
            }
            Message::Kill => self.will_to_live = false,
            Message::Add(user) => self.users.push((*user).clone()),
            Message::Remove(sender) => {
                if let Some(pos) = self.users.iter().position(|x| x.sender == *sender) {
                    self.users.swap_remove(pos);
                }
            }
            _ => (),
        });
        data
    }
}
