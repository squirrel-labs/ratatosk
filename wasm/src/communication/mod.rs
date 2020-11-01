pub mod message_queue;
pub mod sprite;
pub mod synchronization_memory;

#[doc(inline)]
pub use message_queue::{Message, MessageQueue, MESSAGE_QUEUE_ELEMENT_COUNT};
use rask_engine::resources;
use spin::{Mutex, RwLock};
#[doc(inline)]
pub use sprite::Sprite;
#[doc(inline)]
pub use synchronization_memory::{GameState, SynchronizationMemory};

pub static DOUBLE_BUFFER: Mutex<Vec<Sprite>> = Mutex::new(Vec::new());
pub static RESOURCE_TABLE: RwLock<resources::ResourceTable> =
    RwLock::new(resources::ResourceTable::new());
pub static mut SYNCHRONIZATION_MEMORY: SynchronizationMemory = SynchronizationMemory::new();
pub static TEXTURE_IDS: Mutex<resources::TextureIds> = Mutex::new(resources::TextureIds::empty());
// The scaling of the screen rect in relation to the world coordinate system
// 1.0 means the world rect fully contains the screen rect (edge cutting)
// 2.0 means the screen rect fully contains the world rect (letterboxing)
pub(crate) static SCREEN_RECT_SCALE: RwLock<f32> = RwLock::new(1.0);
