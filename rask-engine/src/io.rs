use crate::events::Event;
use crate::resources::registry::{CharacterInfo, ResourceInfo};
use crate::resources::Sprite;
use crate::EngineError;

pub enum Message {
    None,
    SystemInternal,
    Event(Event),
}

pub trait SystemApi {
    fn poll_message(&mut self) -> Result<Message, EngineError>;
    fn fetch_resource(&mut self, info: ResourceInfo) -> Result<(), EngineError>;
    fn fetch_character_resource(&mut self, info: CharacterInfo) -> Result<(), EngineError>;
    fn check_fetched(&self, id: u32) -> bool;
    fn push_sprites(&mut self, sprites: Vec<Sprite>);
    fn send_event(&self, event: Event);
    fn play_sound(&self, id: u32);
    fn stop_sound(&self, id: u32);
}
