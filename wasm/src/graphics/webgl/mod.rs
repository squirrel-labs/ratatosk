mod bindings;
mod error;
mod shader;

#[doc(inline)]
pub use error::WebGl2Error;

use super::GraphicsApi;
use crate::communication::Sprite;
use crate::error::ClientError;
use bindings::Gl2;
use rask_engine::{math::Mat3, resources::Texture};
use std::collections::HashMap;

mod imports {
    extern "C" {
        pub fn get_canvas_size() -> u32;
        pub fn set_canvas_size(w: u32, h: u32);
    }
}

/// This function is used to initialize the canvas size, because it may not be available at context
/// creation.
/// Do not call this function to query size at runtime. This information is already available in
/// the synchronization memory.
fn init_canvas_size() -> (u32, u32) {
    let x = unsafe { imports::get_canvas_size() };
    let (x, y) = (x >> 16, x & 0xffff);
    unsafe { crate::communication::SynchronizationMemory::get_mut() }.canvas_size = (x, y);
    (x, y)
}

fn set_canvas_size(w: u32, h: u32) {
    unsafe { imports::set_canvas_size(w, h) }
}

pub struct WebGl2 {
    gl: Gl2,
    size: (u32, u32),
    canvas_size: (u32, u32),
    program: shader::ShaderProgram,
    textures: HashMap<u32, usize>,
    texture_range_buffer: Vec<f32>,
    texture_layer_buffer: Vec<u32>,
    matrix_buffer: Vec<f32>,
}

impl GraphicsApi for WebGl2 {
    type GraphicsError = WebGl2Error;

    fn new(width: u32, height: u32) -> Result<Self, ClientError> {
        let gl = Gl2;
        let program = shader::ShaderProgram::new(&gl)?;
        gl.create_vao_with_buffer_data(&[
            -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        ])?;
        log::info!("shaders compiled and linked");
        Ok(Self {
            gl,
            size: (width, height),
            canvas_size: init_canvas_size(),
            program,
            textures: HashMap::new(),
            texture_range_buffer: vec![],
            texture_layer_buffer: vec![],
            matrix_buffer: vec![],
        })
    }

    fn update_sprite_vector(&mut self, sprites: &[Sprite]) {}

    fn upload_textures(&mut self, textures: &[(u32, &Texture)]) -> Result<(), ClientError> {
        Ok(())
    }

    fn remove_textures(&mut self) -> Result<(), ClientError> {
        Ok(())
    }

    fn draw(&mut self) -> Result<(), ClientError> {
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
