pub mod message_queue;
pub mod sprite;
pub mod synchronization_memory;

#[doc(inline)]
pub use message_queue::{Message, MessageQueue};
use parking_lot::Mutex;
use parking_lot::RwLock;
use rask_engine::resources;
#[doc(inline)]
pub use sprite::Sprite;
#[doc(inline)]
pub use synchronization_memory::{GameState, SynchronizationMemory};

pub static DOUBLE_BUFFER: Mutex<Vec<Sprite>> = Mutex::new(Vec::new());
pub static RESOURCE_TABLE: RwLock<resources::ResourceTable> =
    RwLock::new(resources::ResourceTable::new());
pub static TEXTURE_IDS: Mutex<resources::TextureIds> = Mutex::new(resources::TextureIds::empty());
