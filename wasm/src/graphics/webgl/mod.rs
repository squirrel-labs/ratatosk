mod bindings;
mod error;
mod shader;

#[doc(inline)]
pub use error::WebGl2Error;

use super::GraphicsApi;
use crate::error::ClientError;
use bindings::Gl2;
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

/// This function is used to initialize the canvas size, because it may not be available at context
/// creation.
/// Do not call this function to query size at runtime. This information is already available in
/// the synchronization memory.
fn set_canvas_size(w: u32, h: u32) {
    unsafe { imports::set_canvas_size(w, h) }
}

pub struct WebGl2 {
    gl: Gl2,
    size: (u32, u32),
    canvas_size: (u32, u32),
}

impl GraphicsApi for WebGl2 {
    type GraphicsError = WebGl2Error;

    fn new(width: u32, height: u32) -> Result<Self, ClientError> {
        let gl = Gl2;
        gl.create_vao_with_buffer_data(&[
            -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        ])?;
        Ok(Self {
            gl,
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
