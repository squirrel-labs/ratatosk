use rask_engine::math;

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub transform: math::Mat3,
    pub tex_id: u32,
}

impl Default for Sprite {
    fn default() -> Self {
        Self::empty()
    }
}

impl Sprite {
    pub fn new(transform: math::Mat3, tex_id: u32) -> Self {
        Self { transform, tex_id }
    }

    pub const fn empty() -> Self {
        Self {
            transform: math::Mat3::new(1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0),
            tex_id: 0,
        }
    }
}
