use wasm_bindgen::prelude::*;

use log::info;
use webhogg_wasm_shared::wasm_log::WasmLog;
use webhogg_wasm_shared::SHARED_ALLOCATION_AREA_START as ADDR;

#[wasm_bindgen]
pub fn init() {
    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .unwrap();
    unsafe {
        crate::ALLOCATOR.reset();
    }
    info!("graphics entry reached");
}

#[wasm_bindgen]
pub fn frame() {
    unsafe {
        let addr = ADDR as *mut u32;
        *addr += 1;
    }
}
