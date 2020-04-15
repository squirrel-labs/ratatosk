//! This module contains the entry points callable from javascript

extern crate console_error_panic_hook;
use std::panic;
use wasm_bindgen::prelude::*;

use crate::context;

/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn initialise_graphics_context(canvas: web_sys::OffscreenCanvas) {
    unsafe {
        rask_wasm_shared::alloc::reset_heap(&crate::ALLOCATOR, log::LevelFilter::Debug);
    }
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    log::info!("graphics entry reached");

    context::set_context(
        context::Context::new(canvas)
            .map_err(|e| panic!("{}", e))
            .unwrap(),
    );
}

/// This function is being exposed to javascript
#[wasm_bindgen]
pub fn draw_frame() {
    let _ = context::context_mut()
        .render()
        .map_err(|e| log::debug!("{}", e));
}
