use crate::state::State;
type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::sprite::*;

pub const GRAPHIC_STACK_SIZE: usize = 0x0010_0000;
pub const ALLOCATOR_AREA_START: usize = GRAPHIC_STACK_SIZE;
pub const SHARED_BUFFER_AREA_START: usize = ALLOCATOR_AREA_START + 0x0010_0000;
pub const LOGIC_ALLOCATION_AREA_START: usize = SHARED_BUFFER_AREA_START + std::mem::size_of::<Buffer>();
pub const GRAPHICS_ALLOCATION_AREA_START: usize = LOGIC_ALLOCATION_AREA_START + 0x0010_0000;
pub const SHARED_ALLOCATION_AREA_START: usize = GRAPHICS_ALLOCATION_AREA_START + 0x0010_0000;

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(SHARED_BUFFER_AREA_START as *mut Buffer) }
}

pub struct SharedHeap {
    notify: bool,
    last_addr: u32,
    animations: Vec<Animation>,
}

impl SharedHeap {
    pub fn is_notify(&self) -> bool {
        self.notify
    }

    pub fn animations_mut(&mut self) -> &mut Vec<Animation> {
        &mut self.animations
    }

    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }
}

pub fn shared_heap() -> &'static mut SharedHeap {
    unsafe { &mut *(SHARED_ALLOCATION_AREA_START as *mut SharedHeap) }
}
