use crate::graphics::GraphicsApi;
use rask_game_engine::math::Mat3;
use webhogg_wasm_shared::error::ClientError;
use webhogg_wasm_shared::get_double_buffer;
use webhogg_wasm_shared::sprite::{Animation, Frame, Sprite};
use webhogg_wasm_shared::state::State;

pub struct Render<T> {
    graphics: T,
    frame_nr: u64,
}

impl<T: GraphicsApi> Render<T> {
    pub fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        T::new(canvas).map(|api| Self {
            graphics: api,
            frame_nr: 0,
        })
    }

    pub fn render(&mut self, animations: &[Animation]) -> Result<(), ClientError> {
        self.graphics
            .ok()
            .map_err(|e| ClientError::WebGlError(format!("WebGl2 error: {}", e)))?;
        self.graphics.clear(&[0.8, 0.05, 0.55])?;
        if self.draw_sprites(animations)? {
            self.frame_nr += 1;
        }
        Ok(())
    }

    pub fn draw_sprites(&mut self, animations: &[Animation]) -> Result<bool, ClientError> {
        if let Some(state) = get_double_buffer().borrow_reader() {
            for sprite in state.get().sprites().iter() {
                self.draw_sprite(sprite, animations)?;
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn draw_sprite(
        &mut self,
        sprite: &Sprite,
        animations: &[Animation],
    ) -> Result<(), ClientError> {
        let frame = sprite
            .get_frame(animations)
            .ok_or(ClientError::ResourceError(
                "could not get animation frame".to_owned(),
            ))?;
        for transformation in frame.transformations().iter() {
            self.graphics.draw_rect(transformation)?;
        }
        Ok(())
    }
}
