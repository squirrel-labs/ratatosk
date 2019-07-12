use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_graphics(canvas: web_sys::OffscreenCanvas) {
    logger::init_logger();
    memory::init_mem();

    match context::graphics::GraphicsContext::from_canvas(canvas) {
        Ok(ctx) => context::set_graphics(ctx),
        Err(e) => {
            error!("graphics {}", e);
            panic!()
        }
    };
}

#[wasm_bindgen]
pub fn loop_graphics(val: usize) {
    context::get_graphics()
        .update()
        .map_err(|e| error!("gaphics loop {}", e))
        .unwrap();
}
