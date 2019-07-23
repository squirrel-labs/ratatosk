//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::websocket::*;
use log::{error, info};
use webhogg_wasm_shared::shared_heap_addr;
use webhogg_wasm_shared::wasm_log::WasmLog;

static mut WS: Option<WebSocketAdapter> = None;

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
        *shared_heap_addr::<u32>(0) = 0;
        WS = Some(
            WebSocketAdapter::new("wss://echo.websocket.org").expect("Websocket creation failed"),
        );
    }
    info!("logic entry reached");
}

/// This function represents a logic tick. State changes caused by network or key events get
/// accumulated over a period of time and processed here
#[wasm_bindgen]
pub fn frame() {
    unsafe {
        let num = *shared_heap_addr::<u32>(0);
        if let Err(res) = WS
            .as_ref()
            .unwrap()
            //.send_u8_arr(format!("num: {}", num).as_bytes_mut())
            .send_str(format!("num: {}", num).as_str())
        {
            error!("websocket not ready: {}", res);
        }
    }
}
