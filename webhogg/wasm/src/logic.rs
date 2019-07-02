use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_logic() -> usize {
    logger::init_logger();
    info!("logic: wasm entry-point reached");

    match context::logic::LogicContext::new() {
        Ok(ctx) => Box::into_raw(Box::new(ctx)) as usize,
        Err(e) => {
            error!("logic {}", e);
            panic!()
        }
    }
}

#[wasm_bindgen]
pub fn loop_logic(val: usize) {
    //debug!("mem1: {}", crate::memory::get_memory_ptr());
    //debug!("val: {}", crate::memory::get_memory());
    
    //info!("");
}
