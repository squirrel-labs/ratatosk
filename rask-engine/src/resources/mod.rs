/*!
The resource management system for the ratatosk game engine.

# Example

```
use lazy_static::lazy_static;
# use rask_engine::resources::*;

lazy_static! {
    static ref TABLE: ResourceTable = unsafe { ResourceTable::new(0, 0) };
}

fn test() {
    unsafe {
        let _texture: &Texture = TABLE.get(0).unwrap();
    }
}
```
*/

mod library;
pub mod sound;
pub mod texture;

#[doc(inline)]
pub use library::*;
#[doc(inline)]
pub use sound::Sound;
#[doc(inline)]
pub use texture::Texture;

pub enum Resource {
    None,
    Texture(Texture),
    Sound(Sound),
    Skeleton(spine::skeleton::Skeleton),
}
