use image::{png::PNGDecoder, ImageDecoder};
use crate::error::ClientError;

use std::convert::TryInto;

pub use image::ColorType;

pub struct Texture {
    raw_data: Vec<u8>,
    w: u32, h: u32,
    colortype: ColorType,
}

impl Texture {
    pub fn from_png_stream<R: std::io::Read>(r: R) -> Result<Self, ClientError> {
        let decoder = PNGDecoder::new(r)
            .map_err(|e| ClientError::ResourceError(format!("png image reading error: {}", e)))?;

        let (w, h) = decoder.dimensions();
        let e = |_| ClientError::ResourceError(format!("invalid image resolution"));
        let (w, h) = (w.try_into().map_err(e)?, h.try_into().map_err(e)?);

        let colortype = decoder.colortype();

        let bytes = decoder
            .read_image()
            .map_err(|e| ClientError::ResourceError(format!("png image decoding error: {}", e)))?;

        Ok(Self {
            raw_data: bytes,
            w, h, colortype,
        })
    }

    pub fn dimension(&self) -> (u32, u32) {
        (self.w, self.h)
    }

    pub fn colortype(&self) -> ColorType {
        self.colortype
    }

    pub fn raw(&self) -> &Vec<u8> {
        &self.raw_data
    }

    pub fn raw_mut(&mut self) -> &mut [u8] {
        &mut self.raw_data
    }
}
