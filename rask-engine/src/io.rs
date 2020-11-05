use crate::events::Event;
use crate::resources::registry::{CharacterInfo, ResourceInfo};
use crate::resources::Sprite;
use crate::EngineError;

#[derive(Debug)]
pub enum Message {
    None,
    SystemInternal,
    Event(Event),
}

pub trait SystemApi: Send + Sync {
    fn poll_message(&mut self) -> Result<Message, EngineError> {
        unimplemented!("poll_message is not implemented for the system api");
    }
    fn fetch_resource(&mut self, _info: ResourceInfo) -> Result<(), EngineError> {
        unimplemented!("fetch_resource is not implemented for the system api");
    }
    fn fetch_character_resource(&mut self, _info: CharacterInfo) -> Result<(), EngineError> {
        unimplemented!("fetch_character_resource is not implemented for the system api");
    }
    fn check_fetched(&self, _id: u32) -> bool {
        unimplemented!("check_fetched is not implemented for the system api");
    }
    fn push_sprites(&mut self, _sprites: Vec<Sprite>) {
        unimplemented!("push_sprites is not implemented for the system api");
    }
    fn push_textures(&mut self, _textures: Vec<u32>) {
        unimplemented!("push_textures is not implemented for the system api");
    }
    fn get_mouse_position(&mut self) -> (i32, i32) {
        unimplemented!("get_mouse_position is not implemented for the system api");
    }
    fn get_canvas_size(&mut self) -> (u32, u32) {
        unimplemented!("get_canvas_size is not implemented for the system api");
    }
    fn send_event(&self, _event: Event) {
        unimplemented!("send_event is not implemented for the system api");
    }
    fn play_sound(&self, _id: u32) {
        unimplemented!("play_sound is not implemented for the system api");
    }
    fn stop_sound(&self, _id: u32) {
        unimplemented!("stop_sound is not implemented for the system api");
    }
}

#[derive(Debug, Default)]
pub struct DummySystemApi;
impl SystemApi for DummySystemApi {}
