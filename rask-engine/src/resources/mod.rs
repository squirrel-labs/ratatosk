/*!
The resource management system for the ratatosk game engine.

# Example

```
use lazy_static::lazy_static;
# use rask_engine::resources::*;

lazy_static! {
    static ref TABLE: ResourceTable = unsafe { ResourceTable::new() };
}

fn test() {
    unsafe {
        let _texture: &Texture = TABLE.get(0).unwrap();
    }
}
```
*/

pub mod character;
pub mod registry;
mod resource_table;
pub mod sound;
pub mod texture;

#[doc(inline)]
pub use character::Character;
#[doc(inline)]
pub use registry::RESOURCE_COUNT;
#[doc(inline)]
pub use resource_table::{GetStore, ResourceTable};
#[doc(inline)]
pub use sound::Sound;
#[doc(inline)]
pub use texture::{Texture, TextureIds};

#[cfg_attr(not(feature = "nightly"), repr(C))]
pub enum Resource {
    None,
    Character(Box<Character>),
    Texture(Texture),
    Sound(Sound),
}
