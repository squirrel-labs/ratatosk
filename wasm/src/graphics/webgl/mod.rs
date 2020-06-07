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
use rask_engine::{
    math::Mat3,
    resources::{GetTextures, Texture, TextureRange},
};
use rect_packer::DensePacker;

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
    canvas_size: (u32, u32),
    // mapping from texture id to texture with texture range and texture layer
    textures: HashMap<(u32, u64), (TextureRange, u32)>,
    sprite_textures: Vec<(u32, u64)>,
    matrix_buffer: Vec<Mat3>,
    max_texture_size: (u32, u32),
    texture_packer: DensePacker,
    layer_index: u32,
    allocated_layers: u32,
}

impl WebGl2 {
    fn generate_texture_buffers(
        &mut self,
        sprites: &[Sprite],
    ) -> Option<(Vec<[f32; 4]>, Vec<u32>)> {
        self.sprite_textures = sprites.iter().map(|s| (s.tex_id, s.tex_sub_id)).collect();
        Some(
            self.sprite_textures
                .iter()
                .map(|i| {
                    self.textures
                        .get(i)
                        .map(|(range, layer)| (range.into_floats(), layer))
                })
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
        shader::ShaderProgram::new(&gl)?;
        gl.create_vao_with_buffer_data(&[
            -1.0, 1.0, 1.0, 1.0, -1.0, -1.0, 1.0, -1.0, -1.0, -1.0, 1.0, 1.0,
        ])?;
        log::info!("shaders compiled and linked");
        let tex_size = gl.get_max_texture_size();
        gl.create_renderbuffer(width, height)?;
        Ok(Self {
            gl,
            canvas_size: init_canvas_size(),
            textures: HashMap::new(),
            sprite_textures: vec![],
            matrix_buffer: vec![],
            max_texture_size: tex_size,
            texture_packer: DensePacker::new(tex_size.0 as i32, tex_size.1 as i32),
            layer_index: 0,
            allocated_layers: 0,
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
                .all(|(&t, s)| s.tex_id == t.0 && s.tex_sub_id == t.1);
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

    fn upload_textures(&mut self, textures: &[(u32, u64, &Texture)]) -> Result<(), ClientError> {
        let (width, height) = self.max_texture_size;
        let mut upload_vec = vec![];
        for texture in textures {
            let rect = self
                .texture_packer
                .pack(texture.2.width() as i32, texture.2.height() as i32, false)
                .or_else(|| {
                    self.texture_packer =
                        rect_packer::DensePacker::new(width as i32, height as i32);
                    self.layer_index += 1;
                    self.texture_packer.pack(
                        texture.2.width() as i32,
                        texture.2.height() as i32,
                        false,
                    )
                })
                .ok_or_else(|| ClientError::WebGlError("texture too large for GPU".to_string()))?;
            let tex = (
                TextureRange::new(
                    (rect.x as u32, rect.y as u32),
                    (rect.width as u32, rect.height as u32),
                    (width, height),
                ),
                self.layer_index,
            );
            self.textures.insert((texture.0, texture.1), tex);
            upload_vec.push((tex, texture.2));
        }
        if self.layer_index + 1 == self.allocated_layers {
            for ((range, layer), tex) in upload_vec {
                self.gl.upload_texture_to_atlas(range, layer, tex);
            }
        } else {
            self.gl
                .realloc_texture_atlas(width, height, self.layer_index + 1)?;
            let guard = crate::communication::RESOURCE_TABLE.read();
            for (&(id, sid), &(range, layer)) in self.textures.iter() {
                let tex = guard.get_texture(id as usize, sid)?;
                self.gl.upload_texture_to_atlas(range, layer, tex);
            }
        }
        self.gl.uniform_texture();
        Ok(())
    }

    fn remove_textures(&mut self) -> Result<(), ClientError> {
        let (width, height) = self.max_texture_size;
        self.texture_packer = rect_packer::DensePacker::new(width as i32, height as i32);
        self.layer_index = 0;
        self.textures.clear();
        Ok(())
    }

    fn draw(&mut self) -> Result<(), ClientError> {
        self.gl
            .draw_arrays_instanced(0, 6, self.matrix_buffer.len() as u32);
        Ok(())
        //todo!("draw function is not implemented yet")
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
