use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::convert::TryInto;

#[derive(Debug)]
pub struct SharedMemory {
    pub num: u32,
}

impl SharedMemory {
    pub fn incr(&mut self) {
        self.num += 1;
    }
    pub fn get(&self) -> u32 {
        self.num
    }
}

static mut MEMORY: SharedMemory = SharedMemory { num: 0 };

#[cfg(target_arch = "wasm32")]
fn raw_ptr() -> *mut u32 {
    unsafe {(&mut MEMORY) as *mut SharedMemory as *mut u32}
}


pub fn increase_memory_val() {
    unsafe { *raw_ptr() += 1 }
}


#[no_mangle]
pub fn get_memory() -> u32 {
    unsafe { *raw_ptr() }
}

#[no_mangle]
pub fn get_memory_ptr() -> usize {
    (unsafe {(&mut MEMORY) as *mut SharedMemory }) as usize
}
