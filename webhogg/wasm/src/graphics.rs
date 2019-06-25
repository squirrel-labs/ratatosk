use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_graphics(canvas: web_sys::OffscreenCanvas) {
    logger::init_logger();
    info!("graphics: wasm entry-point reached");
    //debug!("js value is?: undefined: {}", canvas.is_undefined());
    
    match context::graphics::GraphicsContext::from_canvas(canvas) {
        Ok(ctx) => context::set_graphics(ctx),
        Err(e) => {
            error!("graphics {}", e);
            panic!()
        }
    }
}

#[wasm_bindgen]
pub fn loop_graphics() {
    crate::memory::memory().num += 1;

    context::get_graphics().update()
        .map_err(|e| error!("gaphics loop {}", e))
        .unwrap();
}
