use std::convert::TryInto;

use image::{png::PngDecoder, ImageDecoder};

use crate::error::EngineError;

pub use image::ColorType;

#[derive(Debug, Clone)]
pub struct Texture {
    raw_data: Vec<u8>,
    w: u32,
    h: u32,
    color_type: ColorType,
}

#[derive(Debug, Clone)]
pub struct TextureIds {
    pub reset_notify: u8,
    pub ids: Vec<u32>,
}

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
        let e = |_| EngineError::ResourceType("invalid image resolution".to_owned());
        let (w, h) = (w.try_into().map_err(e)?, h.try_into().map_err(e)?);

        let colortype = decoder.color_type();

        let mut bytes = vec![0; w as usize * h as usize * colortype.bytes_per_pixel() as usize];
        decoder.read_image(&mut bytes)?;

        Ok(Self {
            raw_data: bytes,
            w,
            h,
            color_type: colortype,
        })
    }

    pub fn dimension(&self) -> (u32, u32) {
        (self.w, self.h)
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
