use crate::communication::Sprite;
use crate::mem::DOUBLE_BUFFER_SPRITE_COUNT;

const MAX_SPRITES: usize = DOUBLE_BUFFER_SPRITE_COUNT as usize;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct UnspecificState<S> {
    sprite_len: usize,
    sprites: S,
}

pub type State = UnspecificState<[Sprite; MAX_SPRITES]>;

impl State {
    pub fn sprites(&self) -> &[Sprite] {
        &self.sprites[..self.sprite_len]
    }

    pub fn sprites_mut(&mut self) -> &mut [Sprite] {
        &mut self.sprites[..self.sprite_len]
    }

    pub fn append_sprite(&mut self, sprite: &Sprite) {
        log::trace!(
            "adding sprite ({}/{}): {:?}",
            self.sprite_len,
            MAX_SPRITES,
            sprite
        );
        if self.sprite_len < MAX_SPRITES {
            self.sprites[self.sprite_len] = *sprite;
            self.sprite_len += 1;
        }
    }

    pub const fn empty() -> Self {
        Self {
            sprite_len: 0,
            sprites: [Sprite::empty(); MAX_SPRITES],
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self::empty()
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "State({:?})", self.sprites())
    }
}
