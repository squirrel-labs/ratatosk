use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_logic() {
    logger::init_logger();
    info!("logic: wasm entry-point reached");

    match context::logic::LogicContext::new() {
        Ok(ctx) => context::set_logic(ctx),
        Err(e) => error!("logic {}", e)
    }
}

#[wasm_bindgen]
pub fn loop_logic() {
    info!("num: {}", crate::memory::memory().num);
    //debug!("logic: loopin'");
}
