//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::websocket::*;
use log::{error, info};
use webhogg_wasm_shared::get_double_buffer;
use webhogg_wasm_shared::state::State;
use webhogg_wasm_shared::wasm_log::WasmLog;

/// This function is used to initialize the gamestate, communication
/// with the graphics worker and setup networking
#[wasm_bindgen]
pub fn initialise() {
    unsafe {
        crate::ALLOCATOR.reset();
    }
    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
    {
        let mut writer = get_double_buffer().borrow_writer();
        writer.set(State::default());
    }
    crate::game_context::set_context(
        crate::game_context::GameContext::new()
            .map_err(|e| log::error!("{}", e))
            .unwrap(),
    );
    info!("logic entry reached");
}

/// This function represents a logic tick. State changes caused by network or key events get
/// accumulated over a period of time and processed here
#[wasm_bindgen]
pub fn frame() {
    crate::game_context::context_mut()
        .tick()
        .map_err(|e| log::error!("{}", e))
        .unwrap();
}
