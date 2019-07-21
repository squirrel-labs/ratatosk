//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::websocket::*;
use webhogg_wasm_shared::console_log;
use webhogg_wasm_shared::SHARED_ALLOCATION_AREA_START as ADDR;

/// This function is used to initialize the gamestate, communication
/// with the graphics worker and setup networking
#[wasm_bindgen]
pub fn init() {
    unsafe {
        crate::ALLOCATOR.reset();
    }
    unsafe {
        let addr = ADDR as *mut u32;
        *addr = 0;
        let ws = addr.offset(4 as isize) as *mut WebSocketAdapter;
        *ws = WebSocketAdapter::new("wss://echo.websocket.org").expect("Websocket creation failed");
    }
    console_log!("logic entry reached");
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
            console_log!("websocket not ready: {}", res);
        }
    }
}
