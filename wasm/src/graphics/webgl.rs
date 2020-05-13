use super::webgl_bindings::Gl2;
use super::GraphicsApi;
use crate::error::ClientError;

#[derive(Debug)]
pub enum WebGl2Error {
    InvalidEnum,
    InvalidValue,
    InvalidOperation,
    InvalidFramebufferOperation,
    OutOfMemory,
    UnknownError,
}

impl From<u32> for WebGl2Error {
    fn from(v: u32) -> Self {
        match v {
            Gl2::INVALID_ENUM => WebGl2Error::InvalidEnum,
            Gl2::INVALID_VALUE => WebGl2Error::InvalidValue,
            Gl2::INVALID_OPERATION => WebGl2Error::InvalidOperation,
            Gl2::INVALID_FRAMEBUFFER_OPERATION => WebGl2Error::InvalidFramebufferOperation,
            Gl2::OUT_OF_MEMORY => WebGl2Error::OutOfMemory,
            _ => WebGl2Error::UnknownError,
        }
    }
}

impl std::fmt::Display for WebGl2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WebGl2Error::InvalidEnum => "invalid enum",
                WebGl2Error::InvalidValue => "invalid value",
                WebGl2Error::InvalidOperation => "invalid operation",
                WebGl2Error::InvalidFramebufferOperation => "invalid framebuffer operation",
                WebGl2Error::OutOfMemory => "out of memory",
                WebGl2Error::UnknownError => "unknown webgl2 error",
            }
        )
    }
}

pub struct WebGl2 {
    gl: Gl2,
}

impl GraphicsApi for WebGl2 {
    type GraphicsError = WebGl2Error;

    fn new(width: u32, height: u32) -> Result<Self, ClientError> {
        Ok(Self { gl: Gl2 })
    }

    fn start_frame(&mut self, color: &[f32; 3]) -> Result<(), ClientError> {}
    fn end_frame(&self) -> Result<(), ClientError> {}
    fn draw_rect(&self, mat: &Mat3, tex: u32) -> Result<Option<()>, ClientError> {}
    fn upload_texture(&mut self, texture: &Texture, n: u32) -> Result<(), ClientError> {}
    fn unload_texture(&mut self, id: u32) -> Result<(), ClientError> {}
    fn resize_texture_pool(&mut self, n: u32) -> Result<(), ClientError> {}
    fn set_size(&mut self, w: u32, h: u32) {}
    fn update_size(&mut self, w: u32, h: u32) {}
    fn ok(&self) -> Result<(), Self::GraphicsError> {}
}
