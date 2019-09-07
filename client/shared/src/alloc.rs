/*pub struct AllocatorSettings {
    pub allocator_addr: usize,
    pub allocation_start_address: usize,
}

#[macro_export]
macro_rules! get_allocator {
    () => {
        Allocator(std::marker::PhantomData)
    };
}

pub struct Allocator<Method, Type: SpecificAllocator>(pub std::marker::PhantomData<Type>, pub std::marker::PhantomData<Method>);
pub struct LogicAllocator<Method>(pub _phantom: std::marker::PhantomData<Method>);
pub struct GraphicsAllocator<Method>(pub _phantom: std::marker::PhantomData<Method>);

pub trait SpecificAllocator {
    type A: MutableAllocator;

    fn settings() -> AllocatorSettings;

    fn allocator() -> *mut Self::A {
        Self::settings().allocator_addr as *mut Self::A
    }
}

impl<Method> SpecificAllocator for LogicAllocator<Method> {
    type A = MutableAllocator;
    fn settings() -> AllocatorSettings {
        AllocatorSettings {
            allocator_addr: ALLOCATOR_AREA_START,
            allocation_start_address: LOGIC_ALLOCATION_AREA_START,
        }
    }
}

impl SpecificAllocator for GraphicsAllocator {
    type A = MutableAllocator;
    fn settings() -> AllocatorSettings {
        AllocatorSettings {
            allocator_addr: ALLOCATOR_AREA_START + std::mem::size_of::<Self::A>(),
            allocation_start_address: GRAPHICS_ALLOCATION_AREA_START,
        }
    }
}

/*impl<Type: SpecificAllocator> Allocator<Type> {
    pub unsafe fn reset(&self) {
        *Type::allocator() = MutableAllocator {
            pos: Type::settings().allocation_start_address as usize,
        }
    }
}*/

fn layout_to_size(layout: Layout) -> usize {
    let (size, align) = (layout.size(), layout.align());
    let overhead = size % align;
    size + (if overhead == 0 { 0 } else { align - overhead })
}

unsafe impl<Method: MutableAlloc, Type: SpecificAllocator> GlobalAlloc for Allocator<Method, Type> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        (*Type::allocator()).alloc(_layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        (*Type::allocator()).dealloc(_ptr, _layout)
    }
}

unsafe impl<Method: GlobalAlloc, Type: SpecificAllocator> GlobalAlloc for Allocator<Method, Type> {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        (*Type::allocator()).alloc(_layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        (*Type::allocator()).dealloc(_ptr, _layout)
    }
}

pub trait MutableAllocator {
    fn alloc(&mut self, layout: Layout) -> *mut u8;
    fn dealloc(&mut self, ptr: *mut u8, layout: Layout);
}

pub struct SimpleAllocator {
    pos: usize,
}

impl MutableAllocator for SimpleAllocator {
    unsafe fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let size = layout_to_size(layout);
        let pos = self.pos;
        self.pos += size;
        pos as *mut u8
    }

    #[allow(unused_variables)]
    unsafe fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {}
}*/

pub mod settings {
    use crate::mem::*;
    use std::alloc::{GlobalAlloc, Layout};

    pub trait AllocSettings {
        fn allocator_addr<T: Sized>() -> usize;
        fn allocation_start_address<T: Sized>() -> isize;
    }
    pub struct Logic;
    pub struct Graphics;

    impl AllocSettings for Logic {
        fn allocator_addr<T: Sized>() -> usize {
            ALLOCATOR_AREA_START
        }
        fn allocation_start_address<T: Sized>() -> isize {
            LOGIC_ALLOCATION_AREA_START as isize
        }
    }
    
    impl AllocSettings for Graphics {
        fn allocator_addr<T: Sized>() -> usize {
            ALLOCATOR_AREA_START + std::mem::size_of::<T>()
        }
        fn allocation_start_address<T: Sized>() -> isize {
            GRAPHICS_ALLOCATION_AREA_START as isize
        }
    }
}

use settings::AllocSettings;
use std::alloc::{GlobalAlloc, Layout};

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
    fn init() -> T {
        unsafe {std::mem::zeroed()}
    }
}

pub struct NaiveInitial;
impl<T> Initial<T> for NaiveInitial {}

pub struct Allocator<M, S: AllocSettings, I: Initial<M>>(pub std::marker::PhantomData<M>, pub std::marker::PhantomData<S>, pub std::marker::PhantomData<I>);

impl<M: Sized + 'static, S: AllocSettings, I: Initial<M>> Allocator<M, S, I> {
    fn allocator() -> &'static mut M {
        unsafe { &mut *(S::allocator_addr::<M>() as *mut M) }
    }

    pub fn reset(&self) {
        *Self::allocator() = I::init();
    }
}

use wasm_bindgen::prelude::*;
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace=console, js_name=log)]
    fn lognum(l: usize, n: usize);
}

fn logptr(num: usize, ptr: *mut u8) -> *mut u8 {
    //lognum(num, ptr as usize);
    ptr
}

unsafe impl<M: MutableAlloc + Sized + 'static, S: AllocSettings, I: Initial<M>> GlobalAlloc for Allocator<M, S, I> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        logptr(layout.size(), Self::allocator().alloc(layout).offset(S::allocation_start_address::<M>()))
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        Self::allocator().dealloc(ptr.offset(-S::allocation_start_address::<M>()), layout)
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
        impl Initial<$method> for __Initial { fn init() -> $method { $v } }
        #[global_allocator]
        static $name: Allocator<$method, $setting, __Initial> = Allocator(std::marker::PhantomData, std::marker::PhantomData, std::marker::PhantomData);
    };
    ($name:ident,$method:ty,$setting:ty) => {
        #[global_allocator]
        static $name: Allocator<$method, $setting, NaiveInitial> = Allocator(std::marker::PhantomData, std::marker::PhantomData, std::marker::PhantomData);
    }
}
