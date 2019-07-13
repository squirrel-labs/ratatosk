use wasm_bindgen::prelude::*;

use webhogg_wasm_shared::wasm_log::log;
use webhogg_wasm_shared::ADDR;

#[wasm_bindgen]
pub fn init() {
    log("graphics entry reached");
}

#[wasm_bindgen]
pub fn frame() {
    unsafe {
        let addr = ADDR as *mut u32;
        *addr += 1;
    }
}
