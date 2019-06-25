pub struct SharedMemory {
    pub num: u32,
}

static mut MEMORY: SharedMemory = SharedMemory {
    num: 0
};

pub fn memory() -> &'static mut SharedMemory {
    unsafe { &mut MEMORY }
}
