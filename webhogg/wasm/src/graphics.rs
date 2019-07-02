use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_graphics(canvas: web_sys::OffscreenCanvas) -> usize {
    logger::init_logger();
    info!("graphics: wasm entry-point reached");
    //debug!("js value is?: undefined: {}", canvas.is_undefined());
    
    match context::graphics::GraphicsContext::from_canvas(canvas) {
        Ok(ctx) => Box::into_raw(Box::new(ctx)) as usize,
        Err(e) => {
            error!("graphics {}", e);
            panic!()
        }
    }
}

#[wasm_bindgen]
pub fn loop_graphics(val: usize) {
    //crate::memory::increase_memory_val();
    //debug!("mem2: {}", crate::memory::get_memory_ptr());
    //info!("js value is?: undefined: {}", val.is_undefined());

    //return;

    /*context::get_graphics().update()
        .map_err(|e| error!("gaphics loop {}", e))
        .unwrap();*/
}
