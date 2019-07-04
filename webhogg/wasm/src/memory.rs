use std::convert::TryInto;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

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

pub fn increase_memory_val() {
    unsafe { MEMORY.incr() }
}

#[no_mangle]
pub fn get_memory() -> u32 {
    unsafe { MEMORY.get() }
}

#[no_mangle]
pub fn get_memory_ptr() -> usize {
    (unsafe { (&mut MEMORY) as *mut SharedMemory }) as usize
}
