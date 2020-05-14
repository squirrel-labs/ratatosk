type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::double_buffer::DoubleBuffer;
use crate::message_queue::{Message, MessageQueueElement};
use crate::sprite::Sprite;
use crate::state::{State, UnspecificState};
use const_env::from_env;
use rask_engine::network::packet::GameState;
use rask_engine::resources::Resource;
use std::mem::size_of;

#[cfg(target_arch = "wasm32")]
mod asserts {
    macro_rules! assert_env {
        ($var:expr) => {
            pub const _: &str = env!($var);
        };
    }

    assert_env!("WEE_ALLOC_STATIC_ARRAY_BACKEND_BYTES");
    assert_env!("LOGIC_STACK");
    assert_env!("GRAPHICS_STACK");
    assert_env!("ALLOCATOR");
    assert_env!("GRAPHICS_HEAP");
    assert_env!("SYNCHRONIZATION_MEMORY");
    assert_env!("RESOURCE_TABLE");
    assert_env!("RESOURCE_TABLE_SIZE");
    assert_env!("DOUBLE_BUFFER");
    assert_env!("DOUBLE_BUFFER_SIZE");
    assert_env!("MESSAGE_QUEUE");
    assert_env!("MESSAGE_QUEUE_SIZE");
    assert_env!("LOGIC_HEAP");
}
#[from_env]
/// The position of the stack.
pub const LOGIC_STACK: usize = 0;

#[from_env]
/// The position of the stack.
pub const GRAPHICS_STACK: usize = 0;

#[from_env]
/// The address of the Allocator structures
pub const ALLOCATOR: usize = 0;

#[from_env]
/// The graphics heap address
pub const GRAPHICS_HEAP: usize = 0;

#[from_env]
/// The address memory synchronization area.
/// It contains data needed for synchronization between main thread and logic thread.
pub const SYNCHRONIZATION_MEMORY: usize = 0;

#[from_env]
/// Address of the internal resource library.
pub const RESOURCE_TABLE: usize = 0;
#[from_env]
pub const RESOURCE_TABLE_SIZE: usize = 1024;
pub const RESOURCE_TABLE_ELEMENT_COUNT: usize = RESOURCE_TABLE_SIZE / size_of::<Resource>();

#[from_env]
/// The address of the double buffer (size: target dependent)
pub const DOUBLE_BUFFER: usize = 0;
#[from_env]
pub const DOUBLE_BUFFER_SIZE: usize = 1024;
pub const DOUBLE_BUFFER_SPRITE_COUNT: usize =
    ((DOUBLE_BUFFER_SIZE as i64 - size_of::<DoubleBuffer<()>>() as i64) / 2
        - size_of::<UnspecificState<()>>() as i64) as usize
        / size_of::<Sprite>();

#[from_env]
/// Address of the event queue
pub const MESSAGE_QUEUE: usize = 0;
#[from_env]
pub const MESSAGE_QUEUE_SIZE: usize = 1024;
pub const MESSAGE_QUEUE_ELEMENT_COUNT: usize =
    MESSAGE_QUEUE_SIZE / size_of::<MessageQueueElement<Message>>();

#[from_env]
/// The logic heap address (size: 32MiB)
pub const LOGIC_HEAP: usize = 0;

#[from_env]
/// The  heap size (size: 32MiB)
pub const WEE_ALLOC_STATIC_ARRAY_BACKEND_BYTES: usize = 0;

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(DOUBLE_BUFFER as *mut Buffer) }
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
        while self.last_elapsed_ms == self.elapsed_ms {
            unsafe { wait_until_wake_up_at(SYNCHRONIZATION_MEMORY as *mut i32) }
        }
    }
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    #[link_name = "llvm.wasm.atomic.wait.i32"]
    /// see https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    pub fn llvm_atomic_wait_i32(ptr: *mut i32, exp: i32, timeout: i64) -> i32;

    #[link_name = "llvm.wasm.atomic.notify"]
    /// see https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators
    fn llvm_atomic_notify(ptr: *mut i32, cnt: i32) -> i32;
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
    let res = llvm_atomic_wait_i32(ptr, atomic_read_i32(ptr), -1);
    debug_assert!(res == 0)
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
