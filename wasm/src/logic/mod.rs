//! The GameContext contains the logic state and game engine.
//! Its main purpose is to handle events and execute the game engine.

mod resource_parser;
#[cfg(target_arch = "wasm32")]
use crate::mem;
use crate::{
    communication::{state::State, Message, MessageQueue, DOUBLE_BUFFER},
    error::ClientError,
};
use rask_engine::events::{Event, Key};
use resource_parser::ResourceParser;

#[derive(Debug)]
pub struct LogicContext {
    state: State,
    tick_nr: u64,
    message_queue: MessageQueue<'static>,
    res_parser: ResourceParser,
}

/// The logic context stores everything necessary for event handling and the game engine
impl LogicContext {
    pub fn new(message_queue: MessageQueue<'static>) -> Result<Self, ClientError> {
        Ok(Self {
            state: State::default(),
            tick_nr: 0,
            message_queue,
            res_parser: ResourceParser::new(),
        })
    }

    fn push_state(&mut self) {
        let mut writer = DOUBLE_BUFFER.lock();
        *writer = self.state;
    }

    pub fn tick(&mut self) -> Result<(), ClientError> {
        loop {
            let msg = self.message_queue.pop();
            if let Message::None = msg {
                break;
            }
            log::info!("{:?}", msg);
            self.handle_message(msg)?;
        }

        // TODO: Remove this temporary sprite loading. Replace it with some kind of
        // "resource complete" event
        {
            use rask_engine::resources::GetStore;
            let res = crate::communication::RESOURCE_TABLE.read();
            let tex1: Result<&rask_engine::resources::Texture, _> = res.get(0);
            let tex2: Result<&rask_engine::resources::Texture, _> = res.get(1);
            let texes = (if tex1.is_ok() { 1 } else { 0 }) + (if tex2.is_ok() { 1 } else { 0 });
            if self.state.sprites().len() < texes {
                let (res, id) = match (tex1, tex2) {
                    (Ok(tex1), Err(_)) => (tex1, 0),
                    (Ok(tex1), Ok(tex2)) => {
                        if self.state.sprites().is_empty() {
                            (tex1, 0)
                        } else {
                            (tex2, 1)
                        }
                    }
                    (Err(_), Ok(tex2)) => (tex2, 1),
                    (Err(_), Err(_)) => unreachable!(),
                };
                if res.dimension().0 == 2 {
                    self.state.append_sprite(&crate::communication::Sprite::new(
                        rask_engine::math::Mat3::identity(),
                        id,
                    ));
                } else {
                    self.state.append_sprite(&crate::communication::Sprite::new(
                        rask_engine::math::Mat3::scaling(0.4, 0.4),
                        id,
                    ));
                }
            }
        }

        self.push_state();
        self.tick_nr += 1;
        Ok(())
    }

    fn handle_message(&mut self, message: Message) -> Result<Option<Event>, ClientError> {
        match message {
            Message::KeyDown(modifier, hash) => Ok(Some(Event::KeyDown(modifier, Key::from(hash)))),
            Message::KeyUp(modifier, hash) => Ok(Some(Event::KeyUp(modifier, Key::from(hash)))),
            Message::MouseDown(event) => Ok(Some(Event::MouseDown(event))),
            Message::MouseUp(event) => Ok(Some(Event::MouseUp(event))),
            Message::KeyPress(t, code) => Ok(Some(Event::KeyPress(t as u16, code))),
            Message::RequestAlloc { id, size } => {
                self.res_parser.alloc(id, size);
                Ok(None)
            }
            Message::DoneWritingResource(id) => self.res_parser.parse(id).map(|_| None),
            _ => Err(ClientError::EngineError("Unknown Message Type".into())),
        }
    }
}
