use wasm_bindgen::prelude::*;
use log::*;
use crate::*;

#[wasm_bindgen]
pub fn start_logic() {
    //logger::init_logger();
    info!("logic: wasm entry-point reached");

    return;

    let ctx = match context::logic::LogicContext::new() {
        Ok(ctx) => ctx,
        Err(e) => {
            error!("logic {}", e);
            panic!()
        }
    };
}

#[wasm_bindgen]
pub fn loop_logic(val: usize) -> u32 {
    //debug!("mem1: {}", crate::memory::get_memory_ptr());
    //debug!("val: {}", crate::memory::get_memory());
    
    //info!("");
    
    unsafe {
        let b: Box<u32> = Box::from_raw(64000 as *mut u32);
        let b = Box::leak(b);
        *b
    }
}
