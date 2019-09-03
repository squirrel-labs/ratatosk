//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::websocket::*;
use log::{error, info};
use webhogg_wasm_shared::get_double_buffer;
use webhogg_wasm_shared::wasm_log::WasmLog;

/// This function is used to initialize the gamestate, communication
/// with the graphics worker and setup networking
#[wasm_bindgen]
pub fn initialise() {
    unsafe {
        crate::ALLOCATOR.reset();
    }
    unsafe {
        *shared_heap_addr::<u32>(0) = 0;
        WS = Some(WebSocketAdapter::new("ws://localhost:5001").expect("Websocket creation failed"));
    }
    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
>>>>>>> origin/dev
    info!("logic entry reached");
}

/// This function represents a logic tick. State changes caused by network or key events get
/// accumulated over a period of time and processed here
#[wasm_bindgen]
pub fn frame() {}
