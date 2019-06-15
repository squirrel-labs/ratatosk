pub struct DrawSprite {
    pub pos: (f32, f32),
    pub size: (f32, f32),
}

impl DrawSprite {
    pub fn new(pos: (f32, f32), size: (f32, f32)) -> Self {
        Self {
            pos, size
        }
    }
}
