use rask_game_engine::math;

pub type AnimationId = u32;
pub type FrameId = u32;

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pos: math::Vec2,
    animation_id: AnimationId,
    frame_id: FrameId,
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            pos: math::Vec2::new(0.0, 0.0),
            animation_id: 0,
            frame_id: 0
        }
    }
}

impl Sprite {
    pub fn get_frame<'a>(&self, animations: &'a [Animation]) -> Option<&'a Frame> {
        animations.get(self.animation_id as usize)?.frames.get(self.frame_id as usize)
    }
}

pub struct Animation {
    frames: Vec<Frame>
}

impl Animation {
    pub fn new(frames: Vec<Frame>) -> Self {
        Self { frames }
    }
}

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
