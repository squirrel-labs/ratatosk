/*!
The resource management system for the ratatosk game engine.

# Example

```
use lazy_static::lazy_static;
# use rask_engine::resources::*;

lazy_static! {
    static ref LIB: Library = unsafe { Library::new(0) };
}

fn test() {
    use library::GetStore;

    unsafe {
        let _texture: &Texture = LIB.get(0).unwrap();
    }
}
```
*/

pub mod library;
pub mod sound;
pub mod texture;

#[doc(inline)]
pub use library::Library;
#[doc(inline)]
pub use sound::Sound;
#[doc(inline)]
pub use texture::Texture;

enum Resource {
    None,
    Texture(Texture),
    Sound(Sound),
    Skeleton(spine::skeleton::Skeleton),
}
