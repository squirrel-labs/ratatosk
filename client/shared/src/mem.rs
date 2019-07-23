pub const ALLOCATOR_AREA_START: usize = 0x00000400;
pub const SHARED_ALLOCATION_AREA_START: usize = 0x00100000;
pub const LOGIC_ALLOCATION_AREA_START: usize = 0x00200000;
pub const GRAPHICS_ALLOCATION_AREA_START: usize = 0x00300000;

/// Get the address at the heap with a given offset in bytes
pub fn shared_heap_addr<T>(off: usize) -> *mut T {
    (SHARED_ALLOCATION_AREA_START + off) as *mut T
}
