type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::double_buffer::DoubleBuffer;
use crate::message_queue::{Message, MessageQueueElement};
use crate::sprite::Sprite;
use crate::state::{State, UnspecificState};
use rask_engine::resources::Resource;
use std::mem::size_of;

extern "C" {
    #[no_mangle]
    /// The heap address
    pub static __heap_base: i32;
    pub static __data_end: i32;
    pub static __tls_size: i32;
}
/// The position of the stack.
pub const LOGIC_STACK: usize = 0;

/// The position of the stack.
pub const GRAPHICS_STACK: usize = 0;

/// The address of the Allocator structures
pub const ALLOCATOR: usize = 0;

/// The graphics heap address
pub const GRAPHICS_HEAP: usize = 0;

/// The address memory synchronization area.
/// It contains data needed for synchronization between main thread and logic thread.
pub const SYNCHRONIZATION_MEMORY: usize = 0;

/// Address of the internal resource library.
pub const RESOURCE_TABLE: usize = 0;

pub const RESOURCE_TABLE_SIZE: usize = 1024;
pub const RESOURCE_TABLE_ELEMENT_COUNT: usize = RESOURCE_TABLE_SIZE / size_of::<Resource>();

/// The address of the double buffer (size: target dependent)
pub const DOUBLE_BUFFER: usize = 0;

pub const DOUBLE_BUFFER_SIZE: usize = 1024;
pub const DOUBLE_BUFFER_SPRITE_COUNT: usize =
    ((DOUBLE_BUFFER_SIZE as i64 - size_of::<DoubleBuffer<()>>() as i64) / 2
        - size_of::<UnspecificState<()>>() as i64) as usize
        / size_of::<Sprite>();

/// Address of the event queue
pub const MESSAGE_QUEUE: usize = 0;

pub const MESSAGE_QUEUE_SIZE: usize = 1024;
pub const MESSAGE_QUEUE_ELEMENT_COUNT: usize =
    MESSAGE_QUEUE_SIZE / size_of::<MessageQueueElement<Message>>();

/// The logic heap address (size: 32MiB)
pub const LOGIC_HEAP: usize = 0;

/// The  heap size (size: 32MiB)
pub const WEE_ALLOC_STATIC_ARRAY_BACKEND_BYTES: usize = 0;

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(DOUBLE_BUFFER as *mut Buffer) }
}

#[repr(C)]
pub struct GameState {
    pub player_x: f32,
    pub player_y: f32,
    pub player_state: i32,
}

#[repr(align(4))]
#[repr(C)]
pub struct SynchronizationMemory {
    /// time elapsed since logic thread initialisation in milliseconds
    pub elapsed_ms: i32,
    pub mouse: (i32, i32),
    pub canvas_size: (u32, u32),
    pub player: GameState,
    pub other: GameState,
    last_elapsed_ms: i32,
}

#[allow(clippy::while_immutable_condition)]
impl SynchronizationMemory {
    /// # Safety
    /// This function is safe, if the SYNCHRONIZATION_MEMORY memory address is valid
    /// and is only written to using atomic operations
    pub unsafe fn get() -> &'static Self {
        &*(SYNCHRONIZATION_MEMORY as *const Self)
    }
    /// # Safety
    /// This function is safe, if the SYNCHRONIZATION_MEMORY memory address is valid
    /// and is only written to using atomic operations
    pub unsafe fn get_mut() -> &'static mut Self {
        &mut *(SYNCHRONIZATION_MEMORY as *mut Self)
    }

    pub fn wait_for_main_thread_notify(&mut self) {
        self.last_elapsed_ms = self.elapsed_ms;
        while self.last_elapsed_ms
            == unsafe { atomic_read_i32(SYNCHRONIZATION_MEMORY as *const i32) }
        {
            unsafe { wait_until_wake_up_at(SYNCHRONIZATION_MEMORY as *mut i32) }
        }
    }
}

/*
#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "llvm.wasm.atomic.wait.i32"]
    /// see https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    pub fn llvm_atomic_wait_i32(ptr: *mut i32, exp: i32, timeout: i64) -> i32;

    #[link_name = "llvm.wasm.atomic.notify"]
    /// see https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    fn llvm_atomic_notify(ptr: *mut i32, cnt: i32) -> i32;
}*/
#[cfg(target_arch = "wasm32")]
pub unsafe fn llvm_atomic_wait_i32(ptr: *mut i32, exp: i32, timeout: i64) -> i32 {
    core::arch::wasm32::i32_atomic_wait(ptr, exp, timeout)
}
#[cfg(target_arch = "wasm32")]
unsafe fn llvm_atomic_notify(ptr: *mut i32, cnt: i32) -> u32 {
    core::arch::wasm32::atomic_notify(ptr, 1024)
}

#[allow(unused_variables)]
#[cfg(not(target_arch = "wasm32"))]
unsafe fn llvm_atomic_wait_i32(ptr: *mut i32, exp: i32, timeout: i64) -> i32 {
    -1
}

#[allow(unused_variables)]
#[cfg(not(target_arch = "wasm32"))]
unsafe fn llvm_atomic_notify(ptr: *mut i32, cnt: i32) -> i32 {
    -1
}

/// # Safety
/// This function is not safe, it is a wrapper around raw pointer operations
pub unsafe fn atomic_write_u8(ptr: *mut u8, v: u8) {
    (*(ptr as *mut core::sync::atomic::AtomicU8)).store(v, core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
/// This function is not safe, it is a wrapper around raw pointer operations
pub unsafe fn atomic_read_u8(ptr: *const u8) -> u8 {
    (*(ptr as *const core::sync::atomic::AtomicU8)).load(core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
/// This function is not safe, it is a wrapper around raw pointer operations
pub unsafe fn atomic_read_i32(ptr: *const i32) -> i32 {
    (*(ptr as *const core::sync::atomic::AtomicI32)).load(core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
/// This function is not safe, it is a wrapper around raw pointer operations
pub unsafe fn atomic_write_u32(ptr: *mut u32, v: u32) {
    (*(ptr as *mut core::sync::atomic::AtomicU32)).store(v, core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
/// This function is not safe, it is a wrapper around raw pointer operations
pub unsafe fn atomic_read_u32(ptr: *const u32) -> u32 {
    (*(ptr as *const core::sync::atomic::AtomicU32)).load(core::sync::atomic::Ordering::SeqCst)
}

/// # Safety
/// This function is safe as long the thread waits at a valid memory address
pub unsafe fn wait_until_wake_up_at(ptr: *mut i32) {
    log::error!("{}", SYNCHRONIZATION_MEMORY);
    let mut foo = 0i32;
    let foo = SYNCHRONIZATION_MEMORY as *mut i32;
    //let res = llvm_atomic_wait_i32(&mut foo, 0, 1000 * 1000 * 100);
    let res = llvm_atomic_wait_i32(ptr, atomic_read_i32(ptr), 1000 * 1000 * 100);
    /*
    let res = llvm_atomic_wait_i32(
        SYNCHRONIZATION_MEMORY as *mut i32,
        atomic_read_i32(SYNCHRONIZATION_MEMORY as *const i32),
        1000 * 1000 * 100,
    );*/
    if res != 0 {
        log::error!("res != 0: res={}", res);
    }
    log::debug!("wake up wait");
    //debug_assert_eq!(res, 0)
}

/// performs a notify at a given address and return the count of waiters
/// # Safety
/// This function is safe as long as a valid memory address is specified
pub unsafe fn wake_up_at(ptr: *mut i32) -> bool {
    // Documented at https://tc39.es/ecma262/#sec-atomics.notify
    // and https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators.
    // The notify function wakes all waiters up.

    (llvm_atomic_notify(ptr, -1)) > 0
}
