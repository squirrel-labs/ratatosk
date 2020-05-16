use super::GraphicsApi;
use crate::communication::SynchronizationMemory;
use crate::communication::RESOURCE_TABLE;
use crate::error::ClientError;
use rask_engine::resources::{GetStore, TextureIds};

pub struct Render<T> {
    graphics: T,
    frame_nr: u64,
    used_texture_ids: Vec<u32>,
}

impl<T: GraphicsApi> Render<T> {
    pub fn new() -> Result<Self, ClientError> {
        // TODO: Do not hardcode pixelized framebuffer size
        T::new(160, 90).map(|api| Self {
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
        self.graphics.start_frame()?;
        if self.draw_sprites()? {
            self.frame_nr += 1;
        }
        self.graphics.end_frame()
    }

    pub fn upload_texture(&mut self, id: u32) -> Result<(), ClientError> {
        log::info!("upload texture {} to gpu", id);
        if !self.used_texture_ids.contains(&id) {
            let guard = RESOURCE_TABLE.read();
            let texture = guard.get(id as usize)?;
            self.graphics.upload_texture(texture, id)?;
            self.used_texture_ids.push(id)
        }
        Ok(())
    }

    pub fn unload_texture(&mut self, id: u32) -> Result<(), ClientError> {
        if let Some(index) = self.used_texture_ids.iter().position(|&x| x == id) {
            self.graphics.unload_texture(id)?;
            self.used_texture_ids.swap_remove(index);
            Ok(())
        } else {
            Err(ClientError::ResourceError(
                "Tried to remove non-existent texture".to_string(),
            ))
        }
    }

    pub fn reset_textures(&mut self, used_textures: &TextureIds) -> Result<(), ClientError> {
        for texture in self.used_texture_ids.clone().iter() {
            if !used_textures.ids.contains(texture) {
                self.unload_texture(*texture)?;
            }
        }
        unsafe { *(used_textures.reset_notify as *mut u8) = 0 };
        Ok(())
    }

    pub fn draw_sprites(&mut self) -> Result<bool, ClientError> {
        let used_textures = crate::communication::TEXTURE_IDS.lock();
        if used_textures.reset_notify > 0 {
            self.reset_textures(&used_textures)?;
        }
        let state = *crate::communication::DOUBLE_BUFFER.lock();
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
                    .expect("uploaded texture does unexpectedly not exist");
            }
        }
        Ok(true)
    }
}
