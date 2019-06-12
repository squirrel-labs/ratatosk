use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_graphics() {
    logger::init_logger();
    info!("graphics: wasm entry-point reached");

    match context::graphics::GraphicsContext::from_canvas() {
        Ok(ctx) => context::set_graphics(ctx),
        Err(e) => error!("graphics {}", e)
    }
}

#[wasm_bindgen]
pub fn loop_graphics() {
    debug!("graphics: loopin'");
}
