use crate::state::State;
type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::{sprite::*, texture::*};

const fn KiB(n: usize) -> usize {
    n * 1024
}
const fn MiB(n: usize) -> usize {
    n * KiB(1024)
}

const STACK_ALIGNMENT: usize = 1024 * 63;

pub const GRAPHIC_STACK_SIZE: usize = MiB(4) + STACK_ALIGNMENT;
pub const ALLOCATOR_AREA_START: usize = GRAPHIC_STACK_SIZE;
pub const SHARED_BUFFER_AREA_START: usize = ALLOCATOR_AREA_START + MiB(1);
pub const LOGIC_ALLOCATION_AREA_START: usize =
    SHARED_BUFFER_AREA_START + std::mem::size_of::<Buffer>();
pub const GRAPHICS_ALLOCATION_AREA_START: usize = LOGIC_ALLOCATION_AREA_START + MiB(32);
pub const SHARED_ALLOCATION_AREA_START: usize = GRAPHICS_ALLOCATION_AREA_START + MiB(32);

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(SHARED_BUFFER_AREA_START as *mut Buffer) }
}

pub struct SharedHeap {
    last_addr: u32,
    animations: Vec<Animation>,
    texture_notify: bool,
    textures: Option<Vec<Texture>>,
}

impl SharedHeap {
    pub fn animations_mut(&mut self) -> &mut Vec<Animation> {
        &mut self.animations
    }

    pub fn animations(&self) -> &Vec<Animation> {
        &self.animations
    }

    pub fn unset_texture_notify(&mut self) {
        self.texture_notify = false
    }

    pub fn set_texture_notify(&mut self) {
        self.texture_notify = true
    }

    pub fn get_texture_notify(&mut self) -> bool {
        self.texture_notify
    }

    pub fn textures_mut(&mut self) -> &mut Option<Vec<Texture>> {
        &mut self.textures
    }

    pub fn textures(&self) -> &Option<Vec<Texture>> {
        &self.textures
    }
}

pub fn shared_heap() -> &'static mut SharedHeap {
    unsafe { &mut *(SHARED_ALLOCATION_AREA_START as *mut SharedHeap) }
}

extern "C" {
    #[link_name = "llvm.wasm.atomic.wait.i32"]
    /// see https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    pub fn llvm_atomic_wait_i32(ptr: *mut i32, exp: i32, timeout: i64) -> i32;

    #[link_name = "llvm.wasm.atomic.notify"]
    /// see https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    fn llvm_atomic_notify(ptr: *mut i32, cnt: i32) -> i32;
}

pub unsafe fn atomic_write_u8(ptr: *mut u8, v: u8) {
    (*(ptr as *mut core::sync::atomic::AtomicU8)).store(v, core::sync::atomic::Ordering::SeqCst)
}

pub unsafe fn atomic_read_u8(ptr: *const u8) -> u8 {
    (*(ptr as *const core::sync::atomic::AtomicU8)).load(core::sync::atomic::Ordering::SeqCst)
}

pub unsafe fn atomic_read_i32(ptr: *const i32) -> i32 {
    (*(ptr as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::SeqCst)
}

pub fn wait_until_wake_up_at(ptr: *mut i32) {
    let res = unsafe { llvm_atomic_wait_i32(ptr, atomic_read_i32(ptr), -1) };
    debug_assert!(res == 0)
}

/// performs a notify at a given address and return the count of waiters
pub fn wake_up_at(ptr: *mut i32) -> bool {
    // documented at https://tc39.es/ecma262/#sec-atomics.notify and https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    // the notify function wakes all waiters up 
    (unsafe { llvm_atomic_notify(ptr, -1) }) > 0
}
