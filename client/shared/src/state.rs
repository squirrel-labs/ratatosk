use crate::sprite::Sprite;

const MAX_SPRITES: usize = 64;

pub struct State {
    sprite_len: usize,
    sprites: [Sprite; MAX_SPRITES]
}

impl Default for State {
    fn default() -> Self {
        Self {
            sprite_len: 0,
            sprites: [Sprite::default(); MAX_SPRITES]
        }
    }
}
