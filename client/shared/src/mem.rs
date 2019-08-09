pub const ALLOCATOR_AREA_START: usize = 0x0000_0400;
pub const SHARED_ALLOCATION_AREA_START: usize = 0x0010_0000;
pub const LOGIC_ALLOCATION_AREA_START: usize = 0x0020_0000;
pub const GRAPHICS_ALLOCATION_AREA_START: usize = 0x0030_0000;

use crate::double_buffer::DoubleBuffer;

/// Get the address at the heap with a given offset in bytes
pub unsafe fn shared_heap_addr<T>(off: usize) -> *mut T {
    (SHARED_ALLOCATION_AREA_START + off) as *mut T
}

pub fn get_double_buffer() -> &'static mut DoubleBuffer<u32> {
    unsafe { &mut *shared_heap_addr(0) }
}
