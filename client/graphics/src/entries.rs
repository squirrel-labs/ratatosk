//! This module contains the entry points callable from javascript

use wasm_bindgen::prelude::*;

use crate::context;
use log::info;
use webhogg_wasm_shared::get_double_buffer;
use webhogg_wasm_shared::wasm_log::WasmLog;

#[wasm_bindgen]
pub fn init() {
    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
    unsafe {
        crate::ALLOCATOR.reset();
    }
    context::set_context(
        context::Context::new()
            .map_err(|e| log::error!("{}", e))
            .unwrap(),
    );
    info!("graphics entry reached");
}

#[wasm_bindgen]
pub fn frame() {
    let mut writer = get_double_buffer().borrow_writer();
    writer.set(writer.get() + 1);

    context::context_mut()
        .render()
        .map_err(|e| log::error!("{}", e))
        .unwrap();
}
