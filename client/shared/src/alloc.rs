use settings::AllocSettings;
use std::alloc::{GlobalAlloc, Layout};

pub mod settings {
    use crate::mem::*;

    pub trait AllocSettings {
        fn allocator_addr<T: Sized>() -> usize;
        fn allocation_start_address<T: Sized>() -> isize;
    }
    pub struct Logic;
    pub struct Graphics;

    impl AllocSettings for Logic {
        fn allocator_addr<T: Sized>() -> usize {
            ALLOCATOR
        }
        fn allocation_start_address<T: Sized>() -> isize {
            LOGIC_HEAP as isize
        }
    }

    impl AllocSettings for Graphics {
        fn allocator_addr<T: Sized>() -> usize {
            ALLOCATOR + std::mem::size_of::<T>()
        }
        fn allocation_start_address<T: Sized>() -> isize {
            GRAPHICS_HEAP as isize
        }
    }
}

pub trait MutableAlloc {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout);
}

pub struct SimpleAllocator {
    pos: usize,
}

impl SimpleAllocator {
    fn layout_to_size(layout: Layout) -> usize {
        let (size, align) = (layout.size(), layout.align());
        let overhead = size % align;
        size + (if overhead == 0 { 0 } else { align - overhead })
    }

    pub fn new(pos: usize) -> Self {
        Self { pos }
    }
}

impl MutableAlloc for SimpleAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = Self::layout_to_size(layout);
        let pos = self.pos;
        self.pos += size;
        pos as *mut u8
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {}
}

pub trait Initial<T> {
    unsafe fn init() -> T {
        std::mem::zeroed()
    }
}

pub struct ZeroedInitial;
impl<T> Initial<T> for ZeroedInitial {}

pub struct Allocator<M, S: AllocSettings, I: Initial<M>>(
    pub std::marker::PhantomData<M>,
    pub std::marker::PhantomData<S>,
    pub std::marker::PhantomData<I>,
);

impl<M: Sized + 'static, S: AllocSettings, I: Initial<M>> Allocator<M, S, I> {
    fn allocator() -> &'static mut M {
        unsafe { &mut *(S::allocator_addr::<M>() as *mut M) }
    }

    pub unsafe fn reset(&self) {
        *Self::allocator() = I::init();
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = debug)]
    fn console_debug_u32(il: bool, a: u32, b: u32, s: u32, e: u32, f: u32);
    #[wasm_bindgen(js_namespace = console, js_name = error)]
    fn console_error_u32(a: u32, b: u32, s: u32, e: u32);
}

unsafe impl<M: MutableAlloc + Sized + 'static, S: AllocSettings, I: Initial<M>> GlobalAlloc
    for Allocator<M, S, I>
{
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let out = Self::allocator().alloc(layout);
        let tout = out as usize as u32;
        let size = SimpleAllocator::layout_to_size(layout) as u32;
        let start = S::allocation_start_address::<M>() as usize as u32;
        let heaplen: u32 = env!("WEE_ALLOC_STATIC_ARRAY_BACKEND_BYTES")
            .parse()
            .unwrap();
        if tout + size > start + heaplen || tout < start {
            console_error_u32(size, start, start + heaplen, tout);
        }
        out
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Self::allocator().dealloc(ptr, layout)
    }
}

impl<A: GlobalAlloc> MutableAlloc for A {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        <Self as GlobalAlloc>::alloc(self, layout)
    }

    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        <Self as GlobalAlloc>::dealloc(self, ptr, layout)
    }
}

#[macro_export]
macro_rules! create_allocator {
    ($name:ident,$method:ty,$setting:ty,$v:expr) => {
        struct __Initial;
        impl Initial<$method> for __Initial {
            unsafe fn init() -> $method {
                $v
            }
        }
        #[global_allocator]
        static $name: Allocator<$method, $setting, __Initial> = Allocator(
            std::marker::PhantomData,
            std::marker::PhantomData,
            std::marker::PhantomData,
        );
    };
    ($name:ident,$method:ty,$setting:ty) => {
        #[global_allocator]
        static $name: Allocator<$method, $setting, NaiveInitial> = Allocator(
            std::marker::PhantomData,
            std::marker::PhantomData,
            std::marker::PhantomData,
        );
    };
}

use crate::wasm_log::WasmLog;

pub unsafe fn reset_heap<M: Sized + 'static, S: AllocSettings, I: Initial<M>>(
    alloc: &Allocator<M, S, I>,
    log_level_filter: log::LevelFilter,
) {
    alloc.reset();

    log::set_boxed_logger(Box::new(WasmLog::new()))
        .map(|()| log::set_max_level(log_level_filter))
        .unwrap();
}
