use rask_engine::math;

pub type AnimationId = u32;
pub type FrameId = u32;
pub type TextureId = u32;

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub pos: math::Vec2,
    pub animation_id: AnimationId,
    pub frame_id: FrameId,
    pub tex_id: TextureId,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            pos: math::Vec2::new(0.0, 0.0),
            animation_id: 0,
            frame_id: 0,
            tex_id: 0,
        }
    }
}

impl Sprite {
    pub fn get_animation<'a>(&self, animations: &'a [Animation]) -> Option<&'a Animation> {
        animations.get(self.animation_id as usize)
    }

    pub fn get_frame<'a>(&self, animations: &'a [Animation]) -> Option<&'a Frame> {
        self.get_animation(animations)?.frames.get(self.frame_id as usize)
    }

    pub fn new(pos: math::Vec2, animation_id: AnimationId, frame_id: FrameId, tex_id: TextureId) -> Self {
        Self {
            pos, animation_id, frame_id, tex_id
        }
    }

    pub fn next_frame(&mut self, animations: &[Animation]) -> Option<FrameId> {
        self.frame_id = self.get_animation(animations)?.next(self.frame_id);
        Some(self.frame_id)
    }
}

#[derive(Debug)]
pub struct Animation {
    frames: Vec<Frame>,
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self { frames }
    }

    pub fn next(&self, frame_id: FrameId) -> FrameId {
        match self.frames.len() as u32 {
            0 => 0,
            len if len - 1 <= frame_id => (frame_id + 1) % len,
            _ => frame_id + 1
        }
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
