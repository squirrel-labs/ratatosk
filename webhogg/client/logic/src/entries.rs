use wasm_bindgen::prelude::*;

use webhogg_wasm_shared::wasm_log::log;
use webhogg_wasm_shared::SHARED_ALLOCATION_AREA_START as ADDR;

#[wasm_bindgen]
pub fn init() {
    unsafe {
        crate::ALLOCATOR.reset();
    }
    unsafe {
        let addr = ADDR as *mut u32;
        *addr = 0;
    }
    log("logic entry reached");
}

#[wasm_bindgen]
pub fn frame() {
    unsafe {
        let addr = ADDR as *mut u32;
        log(&format!("num: {}", *addr));
    }
}
