use crate::context::RESOURCE_TABLE;
use crate::graphics::GraphicsApi;
use rask_engine::resources::{registry, GetStore, TextureIds};
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::{get_double_buffer, SynchronizationMemory};

pub struct Render<T> {
    graphics: T,
    frame_nr: u64,
    used_texture_ids: Vec<u32>,
}

impl<T: GraphicsApi> Render<T> {
    pub fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        let factor = rask_engine::math::vec2::Vec2::new(0.2, 0.2);
        T::new(canvas, factor).map(|api| Self {
            graphics: api,
            frame_nr: 0,
            used_texture_ids: vec![],
        })
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        self.graphics
            .ok()
            .map_err(|e| ClientError::WebGlError(format!("WebGl2 error: {}", e)))?;
        let size = (unsafe { SynchronizationMemory::get() }).canvas_size;
        self.graphics.update_size(size.0, size.1);
        self.graphics.start_frame(&[0.0, 0.0, 0.0])?;
        if self.draw_sprites()? {
            self.frame_nr += 1;
        }
        self.graphics.end_frame()
    }

    pub fn upload_texture(&mut self, id: u32) -> Result<(), ClientError> {
        let texture = unsafe { RESOURCE_TABLE.get(id as usize)? };
        self.graphics.resize_texture_pool(id + 1)?;
        self.graphics.upload_texture(texture, id)?;
        if !self.used_texture_ids.contains(&id) {
            self.used_texture_ids.push(id)
        }
        Ok(())
    }

    pub fn unload_texture(&mut self, id: u32) -> Result<(), ClientError> {
        let texture = unsafe { RESOURCE_TABLE.get(id as usize)? };
        self.graphics.resize_texture_pool(id + 1)?;
        self.graphics.upload_texture(texture, id)?;
        Ok(())
    }

    pub fn reset_textures(&mut self, used_textures: &TextureIds) -> Result<(), ClientError> {
        for texture in self.used_texture_ids.clone().iter() {
            if !used_textures.ids.contains(texture) {
                self.unload_texture(*texture)?;
                self.used_texture_ids.swap_remove(*texture as usize);
            }
        }
        unsafe { *(used_textures.reset_notify as *mut u8) = 0 };
        Ok(())
    }

    pub fn draw_sprites(&mut self) -> Result<bool, ClientError> {
        let used_textures = unsafe { RESOURCE_TABLE.get(registry::USED_TEXTURE_IDS.id as usize) };
        if let Err(rask_engine::EngineError::ResourceMissing(_)) = used_textures {
            return Ok(true);
        }
        let used_textures: &TextureIds = used_textures?;
        if used_textures.reset_notify > 0 {
            self.reset_textures(used_textures)?;
        }
        if let Some(state) = get_double_buffer().borrow_reader() {
            let state = state.get();
            let sprites = state.sprites();
            for sprite in sprites {
                if self
                    .graphics
                    .draw_rect(&sprite.transform, sprite.tex_id)?
                    .is_none()
                {
                    self.upload_texture(sprite.tex_id)?;
                    self.graphics
                        .draw_rect(&sprite.transform, sprite.tex_id)?
                        .unwrap();
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
