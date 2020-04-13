//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::context;
use rask_wasm_shared::get_double_buffer;

/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn initialise_graphics_context(canvas: web_sys::OffscreenCanvas) {
    unsafe {
        rask_wasm_shared::alloc::reset_heap(&crate::ALLOCATOR, log::LevelFilter::Debug);
    }

    log::info!("graphics entry reached");

    context::set_context(
        context::Context::new(canvas)
            .map_err(|e| log::error!("{}", e))
            .unwrap(),
    );
}

/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn draw_frame() {
    return;
    context::context_mut()
        .render()
        .map_err(|e| log::error!("{}", e))
        .unwrap();
}
