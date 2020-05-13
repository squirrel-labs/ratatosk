pub mod message_queue;
pub mod sprite;
pub mod state;
pub mod synchronization_memory;
#[doc(inline)]
pub use message_queue::{InboundMessage, MessageQueue, OutboundMessage};
use parking_lot::Mutex;
use parking_lot::RwLock;
use rask_engine::resources;
#[doc(inline)]
pub use sprite::Sprite;
#[doc(inline)]
use state::State;
#[doc(inline)]
pub use synchronization_memory::{GameState, SynchronizationMemory};

pub static DOUBLE_BUFFER: Mutex<State> = Mutex::new(State::empty());
pub static RESOURCE_TABLE: RwLock<resources::ResourceTable> =
    RwLock::new(resources::ResourceTable::new());
