pub mod message_queue;
pub mod sprite;
pub mod state;
use parking_lot::Mutex;
use parking_lot::RwLock;
use rask_engine::resources;
#[doc(inline)]
pub use sprite::Sprite;
#[doc(inline)]
use state::State;

pub static DOUBLE_BUFFER: Mutex<State> = Mutex::new(State::empty());
pub static RESOURCE_TABLE: RwLock<resources::ResourceTable> =
    RwLock::new(resources::ResourceTable::new());
