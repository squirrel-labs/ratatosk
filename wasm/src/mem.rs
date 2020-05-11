type Buffer = crate::double_buffer::DoubleBuffer<State>;

use crate::double_buffer::DoubleBuffer;
use crate::message_queue::{Message, MessageQueueElement};
use crate::sprite::Sprite;
use crate::state::{State, UnspecificState};
use rask_engine::network::packet::GameState;
use parking_lot::RwLock;
use rask_engine::resources::Resource;
use std::mem::size_of;

pub static MEM_ADDRS: RwLock<MemoryAdresses> = RwLock::new(DEFAULT_ADDR);
const DEFAULT_ADDR: MemoryAdresses = MemoryAdresses::empty();

extern "C" {
    #[no_mangle]
    /// The heap address
    pub static __heap_base: i32;
    pub static __data_end: i32;
    pub static __tls_size: i32;
}

const MIN_RESOURCE_TABLE_ELEMENT_COUNT: usize = 128;
const MIN_DOUBLE_BUFFER_SPRITE_COUNT: usize = 128;
const MIN_MESSAGE_QUEUE_ELEMENT_COUNT: usize = 32;
pub const RESOURCE_TABLE_SIZE: usize =
    MIN_RESOURCE_TABLE_ELEMENT_COUNT as usize * size_of::<Resource>();
pub const DOUBLE_BUFFER_SIZE: usize = size_of::<DoubleBuffer<()>>()
    + 2 * size_of::<UnspecificState<()>>()
    + MIN_DOUBLE_BUFFER_SPRITE_COUNT * align_up::<Sprite>(size_of::<Sprite>() as u32) as usize;

pub const MESSAGE_QUEUE_SIZE: usize =
    MIN_MESSAGE_QUEUE_ELEMENT_COUNT as usize * size_of::<MessageQueueElement<Message>>();

const fn align_up<T>(addr: u32) -> u32 {
    let x = std::mem::align_of::<T>() as u32 - 1;
    (addr + x) & !x
}
pub const RESOURCE_TABLE_ELEMENT_COUNT: u32 = (RESOURCE_TABLE_SIZE / size_of::<Resource>()) as u32;
pub const DOUBLE_BUFFER_SPRITE_COUNT: u32 = MIN_DOUBLE_BUFFER_SPRITE_COUNT as u32; // TODO improve memory usage
pub const MESSAGE_QUEUE_ELEMENT_COUNT: u32 = (MESSAGE_QUEUE_SIZE / size_of::<Message>()) as u32;
pub const HEAP_SIZE: u32 = 1024 * 64 * 16;

#[derive(Debug, Clone)]
#[repr(C)]
pub struct MemoryAdresses {
    pub synchronization_memory: u32,
    pub message_queue: u32,
    pub message_queue_length: u32,
    pub double_buffer: u32,
    pub resource_table: u32,
    pub heap_base: u32,
}
impl MemoryAdresses {
    const fn empty() -> Self {
        Self {
            synchronization_memory: 0,
            message_queue: 0,
            message_queue_length: 0,
            double_buffer: 0,
            resource_table: 0,
            heap_base: 0,
        }
    }
    pub fn init(heap_base: u32) {
        let synchronization_memory = align_up::<SynchronizationMemory>(heap_base) as u32;
        let message_queue = align_up::<MessageQueueElement<Message>>(
            synchronization_memory + size_of::<SynchronizationMemory>() as u32,
        );
        let double_buffer =
            align_up::<DoubleBuffer<State>>(message_queue + MESSAGE_QUEUE_SIZE as u32);
        let resource_table = align_up::<Resource>(double_buffer + DOUBLE_BUFFER_SIZE as u32);
        let heap_base = align_up::<u32>(resource_table + RESOURCE_TABLE_SIZE as u32);
        let mem = Self {
            synchronization_memory,
            message_queue,
            message_queue_length: MESSAGE_QUEUE_ELEMENT_COUNT,
            double_buffer,
            resource_table,
            heap_base,
        };
        *(MEM_ADDRS.write()) = mem;
    }
}

pub fn get_double_buffer() -> &'static mut Buffer {
    unsafe { &mut *(MEM_ADDRS.read().double_buffer as *mut Buffer) }
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
        &*(MEM_ADDRS.read().synchronization_memory as *const Self)
    }
    /// # Safety
    /// This function is safe, if the SYNCHRONIZATION_MEMORY memory address is valid
    /// and is only written to using atomic operations
    pub unsafe fn get_mut() -> &'static mut Self {
        &mut *(MEM_ADDRS.read().synchronization_memory as *mut Self)
    }

    pub fn wait_for_main_thread_notify(&mut self) {
        self.last_elapsed_ms = self.elapsed_ms;
        while self.last_elapsed_ms
            == unsafe { atomic_read_i32(MEM_ADDRS.read().synchronization_memory as *const i32) }
        {
            unsafe { wait_until_wake_up_at(MEM_ADDRS.read().synchronization_memory as *mut i32) }
        }
    }
}

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
    let res = llvm_atomic_wait_i32(ptr, atomic_read_i32(ptr), 1000 * 1000 * 1000 * 5);
    debug_assert_eq!(res, 0);
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
