mod bindings;
mod error;
mod shader;

#[doc(inline)]
pub use error::WebGl2Error;

use std::collections::HashMap;

use super::GraphicsApi;
use crate::communication::Sprite;
use crate::error::ClientError;
use bindings::Gl2;
use rask_engine::{math::Mat3, resources::Texture, resources::TextureRange};

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
    // mapping from texture id to texture with texture range and texture layer
    textures: HashMap<(u32, u32), (TextureRange, u32)>,
    sprite_textures: Vec<(u32, u32)>,
    matrix_buffer: Vec<Mat3>,
}

impl WebGl2 {
    fn generate_texture_buffers(
        &mut self,
        sprites: &[Sprite],
    ) -> Option<(Vec<TextureRange>, Vec<u32>)> {
        self.sprite_textures = sprites.iter().map(|s| (s.tex_id, s.attachment)).collect();
        Some(
            self.sprite_textures
                .iter()
                .map(|i| self.textures.get(i).cloned())
                .collect::<Option<Vec<_>>>()?
                .iter()
                .cloned()
                .unzip(),
        )
    }
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
            sprite_textures: vec![],
            matrix_buffer: vec![],
        })
    }

    fn update_sprite_vector(&mut self, sprites: &[Sprite]) -> Result<(), ClientError> {
        if sprites.is_empty() {
            return Ok(());
        }
        if sprites.len() == self.matrix_buffer.len() {
            let keep_textures = self
                .sprite_textures
                .iter()
                .zip(sprites.iter())
                .all(|(&t, s)| (s.tex_id, s.attachment) == t);
            if !keep_textures {
                let (texture_ranges, texture_layers) =
                    self.generate_texture_buffers(sprites).ok_or_else(|| {
                        ClientError::ResourceError(
                            "tried to set sprite texture to non-existent texture".to_string(),
                        )
                    })?;
                self.gl
                    .update_texture_buffer(&texture_ranges, &texture_layers);
            }
            for (i, sprite) in sprites.iter().enumerate() {
                self.matrix_buffer[i] = sprite.transform;
            }
            self.gl.update_matrix_buffer(&self.matrix_buffer);
            Ok(())
        } else {
            self.matrix_buffer = sprites.iter().map(|s| s.transform).collect();
            let (texture_ranges, texture_layers) =
                self.generate_texture_buffers(sprites).ok_or_else(|| {
                    ClientError::ResourceError(
                        "tried to add sprite with non-existent texture".to_string(),
                    )
                })?;
            self.gl
                .allocate_buffers(&self.matrix_buffer, &texture_ranges, &texture_layers)
        }
    }

    fn upload_textures(&mut self, textures: &[((u32, u32), &Texture)]) -> Result<(), ClientError> {
        self.textures = textures.iter().collect();
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
