use crate::graphics::GraphicsApi;
use rask_engine::math::Mat3;
use rask_wasm_shared::error::ClientError;
use rask_wasm_shared::get_double_buffer;
use rask_wasm_shared::sprite::{Animation, Frame, Sprite};
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

    pub fn render(&mut self, animations: &[Animation]) -> Result<(), ClientError> {
        self.graphics
            .ok()
            .map_err(|e| ClientError::WebGlError(format!("WebGl2 error: {}", e)))?;
        //if rask_wasm_shared::mem::shared_heap().get_texture_notify() {
        //    rask_wasm_shared::mem::shared_heap().unset_texture_notify();
        //    self.update_textures()?;
        //}
        self.graphics.start_frame(&[0.8, 0.05, 0.55])?;
        if self.draw_sprites(animations)? {
            self.frame_nr += 1;
        }
        self.graphics.end_frame()
    }

    pub fn update_textures(&mut self) -> Result<(), ClientError> {
        /*if let Some(textures) = rask_wasm_shared::mem::shared_heap().textures_mut() {
            let n = textures.len() as u32;
            self.graphics.resize_texture_pool(n)?;
            if n > self.texture_count {
                for (i, texture) in textures
                    .iter_mut()
                    .skip(self.texture_count as usize)
                    .enumerate()
                {
                    self.graphics.upload_texture(texture, i as u32)?
                }
            }
            self.texture_count = n;
        }*/
        Ok(())
    }

    pub fn draw_sprites(&mut self, animations: &[Animation]) -> Result<bool, ClientError> {
        if let Some(state) = get_double_buffer().borrow_reader() {
            for sprite in state.get().sprites().iter() {
                //log::debug!("draw sprite: {:?}", sprite);
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
            self.graphics
                .draw_rect(&sprite.pos, transformation, sprite.tex_id)?;
        }
        Ok(())
    }
}
