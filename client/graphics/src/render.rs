use crate::context::RESOURCE_TABLE;
use crate::graphics::GraphicsApi;
use rask_engine::resources::GetStore;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;

pub struct Render<T> {
    graphics: T,
    texture_count: u32,
    frame_nr: u64,
}

impl<T: GraphicsApi> Render<T> {
    pub fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        let factor = rask_engine::math::vec2::Vec2::new(0.2, 0.2);
        T::new(canvas, factor).map(|api| Self {
            graphics: api,
            texture_count: 0,
            frame_nr: 0,
        })
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        self.graphics
            .ok()
            .map_err(|e| ClientError::WebGlError(format!("WebGl2 error: {}", e)))?;
        self.graphics.start_frame(&[0.8, 0.05, 0.55])?;
        if self.draw_sprites()? {
            self.frame_nr += 1;
        }
        self.graphics.end_frame()
    }

    pub fn upload_texture(&mut self, id: u32) -> Result<(), ClientError> {
        let texture = unsafe { RESOURCE_TABLE.get(id as usize)? };
        self.graphics.upload_texture(texture, id)?;
        self.texture_count += 1;
        Ok(())
    }

    pub fn draw_sprites(&mut self) -> Result<bool, ClientError> {
        if let Some(state) = get_double_buffer().borrow_reader() {
            let sprites = state.get().sprites();
            self.graphics.resize_texture_pool(sprites.len() as u32)?;
            self.texture_count = 0;
            for sprite in sprites {
                self.upload_texture(sprite.tex_id)?;
                if self.texture_count < 2 {
                    self.upload_texture(sprite.tex_id)?;
                }
                self.graphics.draw_rect(&sprite.transform, sprite.tex_id)?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
