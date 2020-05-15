//! The GameContext contains the logic state and game engine.
//! Its main purpose is to handle events and execute the game engine.

mod resource_parser;
#[cfg(target_arch = "wasm32")]
use crate::mem;
use crate::{
    communication::{state::State, InboundMessage, MessageQueue, DOUBLE_BUFFER},
    error::ClientError,
};
use rask_engine::events::{Event, Key};
use resource_parser::ResourceParser;

#[derive(Debug)]
pub struct LogicContext {
    state: State,
    tick_nr: u64,
    message_queue: MessageQueue<'static, InboundMessage>,
    res_parser: ResourceParser,
}

/// The logic context stores everything necessary for event handling and the game engine
impl LogicContext {
    pub fn new(message_queue: MessageQueue<'static, InboundMessage>) -> Result<Self, ClientError> {
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
            if let InboundMessage::None = msg {
                break;
            }
            log::info!("{:?}", msg);
            self.handle_message(msg).unwrap();
        }

        self.push_state();
        self.tick_nr += 1;
        Ok(())
    }

    fn handle_message(&mut self, message: InboundMessage) -> Result<Option<Event>, ClientError> {
        match message {
            InboundMessage::KeyDown(modifier, hash) => {
                Ok(Some(Event::KeyDown(modifier, Key::from(hash))))
            }
            InboundMessage::KeyUp(modifier, hash) => {
                Ok(Some(Event::KeyUp(modifier, Key::from(hash))))
            }
            InboundMessage::MouseDown(event) => Ok(Some(Event::MouseDown(event))),
            InboundMessage::MouseUp(event) => Ok(Some(Event::MouseUp(event))),
            InboundMessage::KeyPress(t, code) => Ok(Some(Event::KeyPress(t as u16, code))),
            InboundMessage::RequestAlloc { id, size } => {
                self.res_parser.alloc(id, size);
                Ok(None)
            }
            InboundMessage::ResourcePush(id) => self.res_parser.parse(id).map(|_| None),
            _ => Err(ClientError::EngineError("Unknown Message Type".into())),
        }
    }
}
