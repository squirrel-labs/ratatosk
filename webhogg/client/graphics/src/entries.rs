use wasm_bindgen::prelude::*;

use log::info;
use webhogg_wasm_shared::shared_heap_addr;
use webhogg_wasm_shared::wasm_log::WasmLog;

#[wasm_bindgen]
pub fn init() {
    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Debug))
        .unwrap();
    unsafe {
        crate::ALLOCATOR.reset();
    }
    info!("graphics entry reached");
}

#[wasm_bindgen]
pub fn frame() {
    unsafe {
        *shared_heap_addr::<u32>(0) += 1;
    }
}
