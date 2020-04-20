use crate::error::ServerError;
use crate::group::{Message, SendGroup};
use rask_engine::resources::registry;
use rask_engine::{world, EngineError};
use std::io::Read;
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
            .map_err(ServerError::GameCreation)
    }
}

fn add_u32_to_vec(buf: &mut Vec<u8>, n: u32) {
    buf.extend_from_slice(&n.to_le_bytes())
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
            let v = registry::IMAGE1.variant as u32;
            let id = registry::IMAGE1.id;
            let path = registry::IMAGE1.path;

            self.push_resource(v, id, &[path]).unwrap();
            //game.handle_events(messages);
            //game.tick();
            //let b = game.get_broadcast()
            //self.users.iter().foreach(|u| u.sender.send(b));
            let _messages = self.get_messages();
        }
        info!("thread kiled itself");
    }

    fn handle_event(event: rask_engine::events::Event) {}

    // supply paths in order of texture, atlas, skeleton
    fn push_resource(
        &mut self,
        res_type: u32,
        res_id: u32,
        path: &[&str],
    ) -> Result<(), ServerError> {
        let mut buf = Vec::new();
        add_u32_to_vec(&mut buf, 10);
        add_u32_to_vec(&mut buf, res_type);
        add_u32_to_vec(&mut buf, res_id);
        if res_type == 3 {
            let mut res = Vec::new();
            RaskGame::read_to_vec(path[0], &mut res)?;
            let tex_len = res.len();
            RaskGame::read_to_vec(path[1], &mut res)?;
            let atlas_len = res.len() - tex_len;
            RaskGame::read_to_vec(path[2], &mut res)?;
            let skeleton_len = res.len() - (atlas_len + tex_len);
            add_u32_to_vec(&mut buf, tex_len as u32);
            add_u32_to_vec(&mut buf, atlas_len as u32);
            add_u32_to_vec(&mut buf, skeleton_len as u32);
            buf.append(&mut res);
        } else {
            RaskGame::read_to_vec(path[0], &mut buf)?;
        }
        for u in &self.users {
            u.sender.send(ws::Message::from(buf.as_slice()))?;
        }
        Ok(())
    }

    fn read_to_vec(path: &str, buf: &mut Vec<u8>) -> Result<(), ServerError> {
        let mut file = std::fs::File::open(path)?;
        file.read_to_end(buf)?;
        Ok(())
    }

    fn get_messages(&mut self) -> Vec<Message> {
        //  info!("reciver {:#?} is still aive", self.group.receiver);
        let (mut data, control): (Vec<Message>, Vec<Message>) =
            self.group.receiver.try_iter().partition(Message::is_data);
        control.iter().for_each(|x| match x {
            Message::Park => {
                data = Vec::new();
                thread::park();
            }
            Message::Kill => self.will_to_live = false,
            Message::Add(user) => self.users.push(user.clone()),
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
