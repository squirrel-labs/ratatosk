/*!
The resource management system for the ratatosk game engine.

# Example

```
use lazy_static::lazy_static;
# use rask_engine::resources::*;

lazy_static! {
    static ref TABLE: ResourceTable = unsafe { ResourceTable::from_memory(0, 0) };
}

fn test() {
    unsafe {
        let _texture: &Texture = TABLE.get(0).unwrap();
    }
}
```
*/

pub mod character;
mod library;
pub mod registry;
pub mod sound;
pub mod texture;

#[doc(inline)]
pub use library::*;
#[doc(inline)]
pub use character::Character;
#[doc(inline)]
pub use sound::Sound;
#[doc(inline)]
pub use texture::{Texture, TextureIds};

pub enum Resource {
    None,
    Character(Character),
    Texture(Texture),
    TextureIds(TextureIds),
    Sound(Sound),
    Skeleton(spine::skeleton::Skeleton),
}
