use crate::graphics::GraphicsApi;
use webhogg_wasm_shared::error::ClientError;

pub struct Render<T> {
    graphics: T,
}

impl<T: GraphicsApi> Render<T> {
    pub fn new(canvas: web_sys::OffscreenCanvas) -> Result<Self, ClientError> {
        T::new(canvas).map(|api| Self { graphics: api })
    }

    pub fn render(&mut self) -> Result<(), ClientError> {
        match self.graphics.ok() {
            Ok(()) => log::warn!("everything's ok"),
            Err(e) => log::error!("err: {}", e),
        };
        self.graphics.clear()?;
        self.graphics.draw_rect()?;
        Ok(())
    }
}
