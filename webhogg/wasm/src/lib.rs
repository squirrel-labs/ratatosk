use wasm_bindgen::prelude::*;
use log::*;

mod logger;

#[wasm_bindgen]
pub fn start_graphics() {
    logger::init_logger();
    info!("hello from wasm graphics");
}

#[wasm_bindgen]
pub fn start_logic() {
    logger::init_logger();
    debug!("hello from wasm logic");
}
