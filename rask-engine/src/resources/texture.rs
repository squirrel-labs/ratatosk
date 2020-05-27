use crate::error::EngineError;
pub use image::ColorType;
use image::{png::PngDecoder, GenericImageView, ImageDecoder};
use std::convert::TryInto;

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
    start: (f32, f32),
    end: (f32, f32),
}

impl TextureRange {
    pub fn new(start: (f32, f32), end: (f32, f32)) -> Self {
        Self { start, end }
    }

    pub fn new_from_pixels(start: (u32, u32), end: (u32, u32), w: u32, h: u32) -> Self {
        Self {
            start: ((start.0 as f32 / (w as f32)), (start.1 as f32 / (h as f32))),
            end: ((end.0 as f32 / (w as f32)), (end.1 as f32 / (h as f32))),
        }
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
    pub fn from_png_stream<R: std::io::Read>(r: R) -> Result<Self, EngineError> {
        let decoder = PngDecoder::new(r)?;

        let (w, h) = decoder.dimensions();
        let e = |_| EngineError::ResourceType("invalid image resolution".to_string());
        let (w, h) = (w.try_into().map_err(e)?, h.try_into().map_err(e)?);

        let color_type = decoder.color_type();

        let mut bytes = vec![0; w as usize * h as usize * color_type.bytes_per_pixel() as usize];
        decoder.read_image(&mut bytes)?;

        Ok(Self {
            raw_data: bytes,
            width: w,
            height: h,
            color_type,
        })
    }

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
        let raw_data = image.to_bytes();
        Self {
            raw_data,
            width: image.width(),
            height: image.height(),
            color_type: image.color(),
        }
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
