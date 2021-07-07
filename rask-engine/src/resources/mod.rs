/*!
The resource management system for the ratatosk game engine.

# Example

```
use lazy_static::lazy_static;
use rask_engine::resources::registry;
# use rask_engine::resources::*;

lazy_static! {
    static ref TABLE: ResourceTable = unsafe { ResourceTable::new() };
}

fn test() {
    unsafe {
        let _texture: &Texture = TABLE.get(registry::EMPTY).unwrap();
    }
}
```
*/

pub mod character;
pub mod registry;
mod resource_table;
pub mod sound;
pub mod sprite;
pub mod texture;

#[doc(inline)]
pub use character::Character;
#[doc(inline)]
pub use registry::RESOURCE_COUNT;
#[doc(inline)]
pub use resource_table::{GetStore, GetTextures, ResourceTable};
#[doc(inline)]
pub use sound::Sound;
#[doc(inline)]
pub use sprite::Sprite;
#[doc(inline)]
pub use texture::{Texture, TextureIds, TextureRange};

use spin::RwLock;

#[cfg(nightly)]
pub static RESOURCE_TABLE: RwLock<ResourceTable> = RwLock::new(ResourceTable::new());

use lazy_static::lazy_static;
#[cfg(not(nightly))]
lazy_static! {
    pub static ref RESOURCE_TABLE: RwLock<ResourceTable> = RwLock::new(ResourceTable::new());
}

#[cfg_attr(not(feature = "nightly"), repr(C))]
pub enum Resource {
    None,
    Character(Box<Character>),
    Texture(Texture),
    Sound(Sound),
}
