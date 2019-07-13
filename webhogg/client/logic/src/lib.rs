use wasm_bindgen::prelude::*;

use webhogg_wasm_shared::wasm_log::{log, log_num};
use webhogg_wasm_shared::ADDR;

#[wasm_bindgen]
pub fn init() {
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
        log_num(*addr);
    }
}
