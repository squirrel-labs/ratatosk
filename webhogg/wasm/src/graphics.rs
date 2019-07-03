use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_graphics(canvas: web_sys::OffscreenCanvas) {
    unsafe {
        let b: Box<u32> = Box::from_raw(64000 as *mut u32);
        *Box::leak(b) = 0;
    }
    return;
    
    let context = match context::graphics::GraphicsContext::from_canvas(canvas) {
        Ok(ctx) => ctx,
        Err(e) => {
            error!("graphics {}", e);
            panic!()
        }
    };
}

#[wasm_bindgen]
pub fn loop_graphics(val: usize) -> u32 {
    unsafe {
        let b: Box<u32> = Box::from_raw(64000 as *mut u32);
        let b = Box::leak(b);
        *b += 1;
        *b
    }
    //crate::memory::increase_memory_val();
    //debug!("mem2: {}", crate::memory::get_memory_ptr());
    //info!("js value is?: undefined: {}", val.is_undefined());

    //return;

    /*context::get_graphics().update()
        .map_err(|e| error!("gaphics loop {}", e))
        .unwrap();*/
}
