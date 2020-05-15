use super::webgl_bindings::Gl2;
use super::GraphicsApi;
use crate::error::ClientError;
use rask_engine::{math::Mat3, resources::Texture};

mod imports {
    extern "C" {
        pub fn get_canvas_size() -> u32;
        pub fn set_canvas_size(w: u32, h: u32);
    }
}

fn get_canvas_size() -> (u32, u32) {
    let x = unsafe { imports::get_canvas_size() };
    (x >> 16, x & 0xffff)
}

fn set_canvas_size(w: u32, h: u32) {
    unsafe { imports::set_canvas_size(w, h) }
}

#[derive(Debug)]
pub enum WebGl2Error {
    ContextLost,
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
            Gl2::CONTEXT_LOST_WEBGL => WebGl2Error::ContextLost,
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
                WebGl2Error::ContextLost => "lost webgl context",
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
    size: (u32, u32),
    canvas_size: (u32, u32),
}

impl GraphicsApi for WebGl2 {
    type GraphicsError = WebGl2Error;

    fn new(width: u32, height: u32) -> Result<Self, ClientError> {
        Ok(Self {
            gl: Gl2,
            size: (width, height),
            canvas_size: get_canvas_size(),
        })
    }

    fn start_frame(&mut self) -> Result<(), ClientError> {
        log::warn!("do ma warnin'");
        Ok(())
    }

    fn end_frame(&self) -> Result<(), ClientError> {
        Ok(())
    }

    fn draw_rect(&self, mat: &Mat3, tex: u32) -> Result<Option<()>, ClientError> {
        log::debug!("render: {:?}", mat);
        Ok(Some(()))
    }

    fn upload_texture(&mut self, texture: &Texture, id: u32) -> Result<(), ClientError> {
        log::debug!("upload texture {}", id);
        Ok(())
    }

    fn unload_texture(&mut self, id: u32) -> Result<(), ClientError> {
        Ok(())
    }

    fn set_size(&mut self, w: u32, h: u32) {
        self.canvas_size = (w, h);
        set_canvas_size(w, h)
    }

    fn update_size(&mut self, w: u32, h: u32) {
        if (w, h) != self.canvas_size && w != 0 && h != 0 {
            self.set_size(w, h)
        }
    }

    fn ok(&self) -> Result<(), Self::GraphicsError> {
        match self.gl.get_error() {
            Gl2::NO_ERROR => Ok(()),
            err => Err(WebGl2Error::from(err)),
        }
    }
}
