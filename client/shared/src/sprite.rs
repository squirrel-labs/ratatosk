use rask_engine::math;

pub type AnimationId = u32;
pub type FrameId = u32;
pub type TextureId = u32;

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub transform: math::Mat3,
    pub tex_id: TextureId,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            transform: math::Mat3::new(0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0),
            tex_id: 0,
        }
    }
}

impl Sprite {
    pub fn new(transform: math::Mat3, tex_id: TextureId) -> Self {
        Self { transform, tex_id }
    }
}

#[derive(Debug)]
pub struct Frame {
    transformations: Vec<math::Mat3>,
}

impl Frame {
    pub fn transformations(&self) -> &[math::Mat3] {
        &self.transformations
    }
}

impl Frame {
    pub fn new(transformations: Vec<math::Mat3>) -> Self {
        Self { transformations }
    }
}
