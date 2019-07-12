use std::alloc::{GlobalAlloc, Layout};

pub const MIN_ADDR: usize = 64000;

struct Allocator {
    addr_pos: usize
}

#[derive(Clone)]
pub struct TAllocator;

static mut G: Allocator = Allocator {
    addr_pos: MIN_ADDR,
};

unsafe impl GlobalAlloc for TAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        let _size = _layout.size();
        let _align = _layout.align();

        // crate::logger::log_num(_size as u32);
        // crate::logger::log_num(_align as u32);

        let too_much = _size % _align;
        let size = _size + (
                    if too_much == 0
                        { 0 }
                    else
                        { _align - too_much }
                );

        let pos = G.addr_pos;
        G.addr_pos += size;

        pos as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
