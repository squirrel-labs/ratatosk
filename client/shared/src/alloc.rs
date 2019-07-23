use std::alloc::{GlobalAlloc, Layout};

use crate::mem::*;

pub struct AllocatorSettings {
    pub allocator_addr: usize,
    pub allocation_start_address: usize,
}

#[macro_export]
macro_rules! get_allocator {
    () => {
        Allocator {
            _phantom: std::marker::PhantomData,
        }
    };
}

pub struct Allocator<Type: SpecificAllocator> {
    pub _phantom: std::marker::PhantomData<Type>,
}
pub struct LogicAllocator;
pub struct GraphicsAllocator;

pub trait SpecificAllocator {
    fn settings() -> AllocatorSettings;

    fn allocator() -> *mut MutableAllocator {
        Self::settings().allocator_addr as *mut MutableAllocator
    }
}

impl SpecificAllocator for LogicAllocator {
    fn settings() -> AllocatorSettings {
        AllocatorSettings {
            allocator_addr: ALLOCATOR_AREA_START,
            allocation_start_address: LOGIC_ALLOCATION_AREA_START,
        }
    }
}

impl SpecificAllocator for GraphicsAllocator {
    fn settings() -> AllocatorSettings {
        AllocatorSettings {
            allocator_addr: ALLOCATOR_AREA_START + std::mem::size_of::<MutableAllocator>(),
            allocation_start_address: GRAPHICS_ALLOCATION_AREA_START,
        }
    }
}

pub struct MutableAllocator {
    pos: usize,
}

impl<Type: SpecificAllocator> Allocator<Type> {
    pub unsafe fn reset(&self) {
        *Type::allocator() = MutableAllocator {
            pos: Type::settings().allocation_start_address as usize,
        }
    }
}

fn layout_to_size(layout: Layout) -> usize {
    let (size, align) = (layout.size(), layout.align());
    let overhead = size % align;
    size + (if overhead == 0 { 0 } else { align - overhead })
}

unsafe impl<Type: SpecificAllocator> GlobalAlloc for Allocator<Type> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        (*Type::allocator()).alloc(_layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        (*Type::allocator()).dealloc(_ptr, _layout)
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
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {}
}
