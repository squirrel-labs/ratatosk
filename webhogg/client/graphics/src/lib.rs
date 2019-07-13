extern crate webhogg_wasm_shared;

use wasm_bindgen::prelude::*;

use webhogg_wasm_shared::wasm_log::log;

#[wasm_bindgen]
pub fn init() {
    log("graphics entry reached");
}
