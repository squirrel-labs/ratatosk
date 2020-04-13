type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::double_buffer::DoubleBuffer;
use crate::sprite::*;
use crate::state::State;
use const_env::from_env;
use rask_engine::resources::{Resource, ResourceTable};
use std::mem::size_of;

macro_rules! assert_env {
    ($var:expr) => {
        const _: &str = env!($var);
    };
}

#[from_env]
/// The position of the stack.
pub const LOGIC_STACK: usize = 0;
assert_env!("LOGIC_STACK");

#[from_env]
/// The position of the stack.
pub const GRAPHICS_STACK: usize = 0;
assert_env!("GRAPHICS_STACK");

#[from_env]
/// The address of the Allocator structures
pub const ALLOCATOR: usize = 0;
assert_env!("ALLOCATOR");

#[from_env]
/// The graphics heap address
pub const GRAPHICS_HEAP: usize = 0;
assert_env!("GRAPHICS_HEAP");

#[from_env]
/// The address memory synchronization area. (size: 1MiB)
/// It contains data needed for synchronization between main thread and logic thread.
/// This address must currently be 0x50fc00.
/// On change you have to modify the corresponding js file.
pub const SYNCHRONIZATION_MEMORY: usize = 0;
assert_env!("SYNCHRONIZATION_MEMORY");

#[from_env]
/// Address of the internal resource library.
pub const RESOURCE_TABLE: usize = 0;
assert_env!("RESOURCE_TABLE");
#[from_env]
pub const RESOURCE_TABLE_SIZE: usize = 0;
assert_env!("RESOURCE_TABLE_SIZE");
pub const RESOURCE_TABLE_ELEMENT_COUNT: usize = (RESOURCE_TABLE_SIZE as i64
    - size_of::<ResourceTable>() as i64) as usize
    / size_of::<Resource>();

#[from_env]
/// The address of the double buffer (size: target dependent)
pub const DOUBLE_BUFFER: usize = 0;
assert_env!("DOUBLE_BUFFER");
#[from_env]
pub const DOUBLE_BUFFER_SIZE: usize = 0;
assert_env!("DOUBLE_BUFFER_SIZE");
pub const DOUBLE_BUFFER_ELEMENT_COUNT: usize =
    (DOUBLE_BUFFER_SIZE as i64 - size_of::<DoubleBuffer<State>>() as i64) as usize
        / size_of::<Sprite>();

#[from_env]
/// Address of the event queue
pub const MESSAGE_QUEUE: usize = 0;
assert_env!("MESSAGE_QUEUE");
#[from_env]
pub const MESSAGE_QUEUE_SIZE: usize = 0;
assert_env!("MESSAGE_QUEUE_SIZE");
pub const MESSAGE_QUEUE_ELEMENT_COUNT: usize = (MESSAGE_QUEUE_SIZE as i32
    - size_of::<MessageQueue<u8>>() as i32) as usize
    / size_of::<MessageQueueElement<u8>>();

#[from_env]
/// The logic heap address (size: 32MiB)
pub const LOGIC_HEAP: usize = 0;
assert_env!("LOGIC_HEAP");

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(DOUBLE_BUFFER as *mut Buffer) }
}

#[repr(align(4))]
pub struct SynchronizationMemory {
    /// time elapsed since logic thread initialisation in milliseconds
    pub elapsed_ms: i32,
    last_elapsed_ms: i32,
    pub mouse_x: f32,
    pub mouse_y: f32,
}

impl SynchronizationMemory {
    pub unsafe fn get() -> &'static Self {
        &*(SYNCHRONIZATION_MEMORY as *const Self)
    }

    pub unsafe fn get_mut() -> &'static mut Self {
        &mut *(SYNCHRONIZATION_MEMORY as *mut Self)
    }

    pub fn wait_for_main_thread_notify(&mut self) {
        self.last_elapsed_ms = self.elapsed_ms;
        while self.last_elapsed_ms == self.elapsed_ms {
            wait_until_wake_up_at((&mut self.elapsed_ms) as *mut i32)
        }
    }
}

#[repr(align(4))]
struct MessageQueueElement<T: Sized + Clone> {
    reading: u8,
    writing: u8,
    payload: T,
}

impl<T: Sized + Clone> MessageQueueElement<T> {
    fn set_reading(&mut self, val: u8) {
        unsafe { atomic_write_u8(&mut self.reading, val) }
    }

    fn get_writing(&self) -> u8 {
        unsafe { atomic_read_u8(&self.writing) }
    }
    fn read(&mut self) -> Option<T> {
        self.set_reading(1);
        if self.get_writing() == 0 {
            let e = self.payload.clone();
            self.set_reading(0);
            Some(e)
        } else {
            None
        }
    }
}

#[repr(align(4))]
pub struct MessageQueue<T: Sized + Clone> {
    /// the index of the next element to be written
    writer_index: u32,
    /// the index of the next element to be read
    reader_index: u32,
    _phantom: core::marker::PhantomData<T>,
}

impl<T: Sized + Clone> MessageQueue<T> {
    pub fn length() -> usize {
        MESSAGE_QUEUE_ELEMENT_COUNT
    }

    fn mem_size() -> usize {
        core::mem::size_of::<T>() * Self::length() + core::mem::size_of::<Self>()
    }

    unsafe fn get_mut(&mut self, n: usize) -> Option<&mut MessageQueueElement<T>> {
        core::slice::from_raw_parts_mut(
            (self as *mut Self as *mut u8).offset(core::mem::size_of::<Self>() as isize)
                as *mut MessageQueueElement<T>,
            Self::length(),
        )
        .get_mut(n)
    }

    pub fn pop(&mut self) -> Option<T> {
        let e = unsafe { self.get_mut(self.reader_index as usize)? };
        let e = e.read()?;
        self.reader_index += 1;
        if self.reader_index as usize >= Self::length() {
            self.reader_index = 0;
        }
        Some(e)
    }
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

pub unsafe fn atomic_write_u32(ptr: *mut u32, v: u32) {
    (*(ptr as *mut core::sync::atomic::AtomicU32)).store(v, core::sync::atomic::Ordering::SeqCst)
}

pub unsafe fn atomic_read_u32(ptr: *const u32) -> u32 {
    (*(ptr as *const core::sync::atomic::AtomicU32)).load(core::sync::atomic::Ordering::SeqCst)
}

pub fn wait_until_wake_up_at(ptr: *mut i32) {
    let res = unsafe { llvm_atomic_wait_i32(ptr, atomic_read_i32(ptr), -1) };
    debug_assert!(res == 0)
}

/// performs a notify at a given address and return the count of waiters
pub fn wake_up_at(ptr: *mut i32) -> bool {
    // Documented at https://tc39.es/ecma262/#sec-atomics.notify
    // and https://github.com/WebAssembly/threads/blob/master/proposals/threads/Overview.md#wait-and-notify-operators.
    // The notify function wakes all waiters up.

    (unsafe { llvm_atomic_notify(ptr, -1) }) > 0
}
