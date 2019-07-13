use std::alloc::{GlobalAlloc, Layout};

pub struct MutableAllocator {
    pos: usize,
}

pub struct Allocator {
    pub pos: usize,
    pub mem0: usize
}

impl Allocator {
    unsafe fn addr(&self) -> *mut MutableAllocator {
        self.pos as *mut MutableAllocator
    }

    pub unsafe fn reset(&self) {
        *self.addr() = MutableAllocator {
            pos: self.mem0,
        }
    }
}

fn layout_to_size(layout: Layout) -> usize {
    let (size, align) = (layout.size(), layout.align());
    let overhead = size % align;
    size + (if overhead == 0 { 0 } else { align - overhead })
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        (*self.addr()).alloc(_layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        (*self.addr()).dealloc(_ptr, _layout)
    }
}

impl MutableAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout_to_size(layout);
        let pos = self.pos;
        self.pos += size;
        pos as *mut u8
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
    }
}
