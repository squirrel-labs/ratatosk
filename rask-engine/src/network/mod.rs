pub mod packet;
pub mod protocol;

#[derive(Debug)]
#[repr(C)]
/// GameState contains data to be sent over the network and is read by `main.js`.
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    /// Encodes actions the player takes + status effects, e.g. poisoned.
    pub player_state: i32,
}

impl GameState {
    pub const fn new() -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_state: 0,
        }
    }
}
