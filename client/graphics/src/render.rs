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
        Ok(())
    }
}
