use super::resource_parser::ResourceParser;
use crate::{
    communication::{
        Message, MessageQueue, Sprite, DOUBLE_BUFFER, SYNCHRONIZATION_MEMORY, TEXTURE_IDS,
    },
    error::ClientError,
};
use rask_engine::{
    events::{Event, Key},
    io,
    resources::registry::{CharacterInfo, ResourceInfo},
    resources::GetStore,
    EngineError,
};

pub struct SystemIO {
    pub message_queue: MessageQueue,
    res_parser: ResourceParser,
}

/// The logic context stores everything necessary for event handling and the game engine.
impl SystemIO {
    pub fn new() -> Result<Self, ClientError> {
        let res_parser = ResourceParser::new();
        Ok(Self {
            message_queue: MessageQueue::new(),
            res_parser,
        })
    }
    fn handle_message(&mut self, message: Message) -> Result<Option<Event>, EngineError> {
        log::info!("message {:?}", message);
        match message {
            Message::KeyDown(modifier, hash) => Ok(Some(Event::KeyDown(modifier, Key::from(hash)))),
            Message::KeyUp(modifier, hash) => Ok(Some(Event::KeyUp(modifier, Key::from(hash)))),
            Message::MouseDown(event) => Ok(Some(Event::MouseDown(event))),
            Message::MouseUp(event) => Ok(Some(Event::MouseUp(event))),
            Message::KeyPress(t, code) => Ok(Some(Event::KeyPress(t as u16, code))),
            Message::AudioLoaded(id) => {
                let mut res = crate::communication::RESOURCE_TABLE.write();
                res.store(rask_engine::resources::Sound, id as usize)?;
                Ok(None)
            }
            Message::RequestAlloc { id, size } => self.res_parser.alloc(id, size).map(|_| None),
            Message::DoneWritingResource(id) => self.res_parser.parse(id).map(|_| None),
            _ => Err(EngineError::Network("Unknown Message Type".into())),
        }
    }
}

impl rask_engine::io::SystemApi for SystemIO {
    fn poll_message(&mut self) -> Result<io::Message, EngineError> {
        let msg = self.message_queue.pop();
        if let Message::None = msg {
            return Ok(io::Message::None);
        }
        Ok(match self.handle_message(msg)? {
            None => io::Message::SystemInternal,
            Some(event) => io::Message::Event(event),
        })
    }
    fn fetch_resource(&mut self, info: ResourceInfo) -> Result<(), EngineError> {
        self.res_parser.fetch_resource(info)
    }
    fn fetch_character_resource(&mut self, info: CharacterInfo) -> Result<(), EngineError> {
        self.res_parser.fetch_character_resource(info)
    }
    fn check_fetched(&self, id: u32) -> bool {
        let res = crate::communication::RESOURCE_TABLE.read();
        res.resource_present(id as usize)
    }
    fn push_sprites(&mut self, sprites: Vec<Sprite>) {
        *DOUBLE_BUFFER.lock() = sprites;
    }
    fn push_textures(&mut self, tex_ids: Vec<u32>) {
        let mut tex = &mut *TEXTURE_IDS.lock();
        tex.ids = tex_ids;
        tex.reset_notify = 1;
    }
    fn get_mouse_position(&self) -> (i32, i32) {
        unsafe { SYNCHRONIZATION_MEMORY.mouse }
    }
    fn get_canvas_size(&self) -> (u32, u32) {
        unsafe { SYNCHRONIZATION_MEMORY.canvas_size }
    }
    fn send_event(&self, event: Event) {
        Message::EngineEvent(event).send();
    }
    fn play_sound(&mut self, id: u32) {
        Message::PlaySound(id).send();
    }
    fn stop_sound(&mut self, id: u32) {
        Message::StopSound(id).send();
    }
}
