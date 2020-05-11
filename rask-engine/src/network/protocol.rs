pub type WsOptcode = u32;
pub type JsOptcode = u32;

pub mod websocket_opt {
    pub const PUSH_RESOURCE: u32 = 10;
    pub const PUSH_GAMESTATE: u32 = 11;
    pub const PUSH_ENGINE_EVENT: u32 = 12;
    pub const PUSH_SERVER_EVENT: u32 = 13;
}
pub mod resource_types {
    pub const TEXTURE: u32 = 2;
    pub const CHARACTER: u32 = 3;
    pub const SOUND: u32 = 4;
}
pub mod javascript;
