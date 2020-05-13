pub mod message_queue;
pub mod sprite;
pub mod state;
use parking_lot::Mutex;
pub use sprite::Sprite;
use state::State;

pub static DOUBLE_BUFFER: Mutex<State> = Mutex::new(State::empty());
