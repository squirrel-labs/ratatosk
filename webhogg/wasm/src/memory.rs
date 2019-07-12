use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::convert::TryInto;

#[derive(Debug, Default)]
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

static mut MEMORY: Option<SharedMemory> = None;

pub fn init_mem() {
    unsafe { MEMORY = Some(SharedMemory::default()); }
}

pub fn get_mem() -> u32 {
    unsafe { MEMORY.as_mut().unwrap().num }
}

pub fn inc_mem() {
    unsafe { MEMORY.as_mut().unwrap().num += 1 }
}

pub fn set_mem(n: u32) {
    unsafe { MEMORY.as_mut().unwrap().num = n }
}
