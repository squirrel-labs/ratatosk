//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::websocket::*;
use log::{error, info};
use webhogg_wasm_shared::wasm_log::WasmLog;
use webhogg_wasm_shared::SHARED_ALLOCATION_AREA_START as ADDR;

/// This function is used to initialize the gamestate, communication
/// with the graphics worker and setup networking
#[wasm_bindgen]
pub fn init() {
    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
    unsafe {
        crate::ALLOCATOR.reset();
    }
    unsafe {
        let addr = ADDR as *mut u32;
        *addr = 0;
        let ws = addr.offset(4 as isize) as *mut WebSocketAdapter;
        *ws = WebSocketAdapter::new("wss://echo.websocket.org").expect("Websocket creation failed");
    }
    info!("logic entry reached");
}

/// This function represents a logic tick. State changes caused by network or key events get
/// accumulated over a period of time and processed here
#[wasm_bindgen]
pub fn frame() {
    unsafe {
        let addr = ADDR as *mut u32;
        //log(&format!("num: {}", *addr));
        let ws = addr.offset(4 as isize) as *mut WebSocketAdapter;
        if let Err(res) = (*ws).send_str(format!("num: {}", *addr).as_str()) {
            error!("websocket not ready: {}", res);
        }
    }
}
