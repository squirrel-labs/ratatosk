use crate::mem::DOUBLE_BUFFER_SPRITE_COUNT;
use crate::sprite::Sprite;

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
        if self.sprite_len < MAX_SPRITES {
            self.sprites[self.sprite_len] = *sprite;
            self.sprite_len += 1;
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            sprite_len: 0,
            sprites: [Sprite::default(); MAX_SPRITES],
        }
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "State({:?})", self.sprites())
    }
}
