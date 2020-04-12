use crate::graphics::GraphicsApi;
use rask_engine::math::Mat3;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::sprite::{Frame, Sprite};
use rask_wasm_shared::state::State;
use rask_wasm_shared::texture::Texture;

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

    pub fn draw_sprites(&mut self) -> Result<bool, ClientError> {
        if let Some(state) = get_double_buffer().borrow_reader() {
            for sprite in state.get().sprites().iter() {
                //log::debug!("draw sprite: {:?}", sprite);
                self.graphics.draw_rect(&sprite.transform, sprite.tex_id)?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
