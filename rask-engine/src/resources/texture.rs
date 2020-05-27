use crate::error::EngineError;
pub use image::ColorType;
use image::GenericImageView;

#[derive(Debug, Clone)]
pub struct Texture {
    raw_data: Vec<u8>,
    width: u32,
    height: u32,
    color_type: ColorType,
}

#[derive(Debug, Clone)]
pub struct TextureIds {
    pub reset_notify: u8,
    pub ids: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextureRange {
    pub start: (u32, u32),
    pub size: (u32, u32),
    pub target_size: (u32, u32),
}

impl TextureRange {
    pub fn new(start: (u32, u32), size: (u32, u32), target_size: (u32, u32)) -> Self {
        Self {
            start,
            size,
            target_size,
        }
    }

    pub fn into_floats(self) -> (f32, f32, f32, f32) {
        (
            (self.start.0 as f32 / (self.target_size.0 as f32)),
            (self.start.1 as f32 / (self.target_size.1 as f32)),
            (self.size.0 as f32 / (self.target_size.0 as f32)),
            (self.size.1 as f32 / (self.target_size.1 as f32)),
        )
    }
}

#[cfg(feature = "nightly")]
impl TextureIds {
    pub const fn empty() -> Self {
        Self {
            reset_notify: 0,
            ids: vec![],
        }
    }
}

impl Texture {
    pub fn form_raw_parts(
        raw_data: Vec<u8>,
        width: u32,
        height: u32,
        color_type: ColorType,
    ) -> Self {
        Self {
            raw_data,
            width,
            height,
            color_type,
        }
    }

    pub fn from_dynamic_image(image: image::DynamicImage) -> Self {
        let raw_data = image.to_rgba().inner().to_vec();
        Self {
            raw_data,
            width: image.width(),
            height: image.height(),
            color_type: image.color(),
        }
    }

    pub fn from_memory(image: &[u8]) -> Result<Self, EngineError> {
        Ok(Self::from_dynamic_image(image::load_from_memory(image)?))
    }

    pub fn dimension(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn color_type(&self) -> ColorType {
        self.color_type
    }

    pub fn raw(&self) -> &[u8] {
        &self.raw_data
    }

    pub fn raw_mut(&mut self) -> &mut [u8] {
        &mut self.raw_data
    }
}
