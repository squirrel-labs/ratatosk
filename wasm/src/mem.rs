//! This modle abstracts some of the raw memory interaction and calculates the offsets of shared
//! memory constructs
//!
use crate::message_queue::{Message, MessageQueueElement};
use parking_lot::RwLock;
use rask_engine::network::packet::GameState;
use rask_engine::resources::Resource;
use std::mem::size_of;

/// All ofsets into static memory are stored here
/// # Safety
/// This may only be used, once it is initialized by the crate::entries::init(heap_base: u32) call
static MEM_ADDRS: RwLock<MemoryAdresses> = RwLock::new(DEFAULT_ADDR);
const DEFAULT_ADDR: MemoryAdresses = MemoryAdresses::empty();

const MIN_RESOURCE_TABLE_ELEMENT_COUNT: usize = 128;
const MIN_MESSAGE_QUEUE_ELEMENT_COUNT: usize = 32;
/// The size of the resource_table in bytes
pub const RESOURCE_TABLE_SIZE: usize =
    MIN_RESOURCE_TABLE_ELEMENT_COUNT as usize * size_of::<Resource>();
/// The size of the message_queue in bytes
pub const MESSAGE_QUEUE_SIZE: usize =
    MIN_MESSAGE_QUEUE_ELEMENT_COUNT as usize * size_of::<MessageQueueElement<Message>>();

/// Align given memory address up to the alignment of T
/// # Example
/// let a = align_up<u32>(1);
/// assert_eq!(a, 4);
pub const fn align_up<T>(addr: usize) -> usize {
    let x = std::mem::align_of::<T>() - 1;
    (addr + x) & !x
}
/// The number of resources that can be stored in the table
pub const RESOURCE_TABLE_ELEMENT_COUNT: u32 = (RESOURCE_TABLE_SIZE / size_of::<Resource>()) as u32;
/// The number of sprites that can be stored in the graphics logic exchange buffer
pub const DOUBLE_BUFFER_SPRITE_COUNT: u32 = 128;
/// The number of Messages that can be sent by the main thread befor the logic thread has to pop
/// messages to avoid data loss
pub const MESSAGE_QUEUE_ELEMENT_COUNT: u32 = (MESSAGE_QUEUE_SIZE / size_of::<Message>()) as u32;
/// The size of the wasm heap. This has to fit into the imported memory to avoid out ouf bound
/// memory access
pub const HEAP_SIZE: u32 = 1024 * 64 * 16;

lazy_static! {
    /// Location of the synchronization memory
    /// Only valid if `entries::init()` was called prior to the first acess
    pub static ref SYNCHRONIZATION_MEMORY: usize = {
        let sync = MEM_ADDRS.read().synchronization_memory;
        if sync == 0 {
            panic!("Tried to read from MEM_ADDRS prior to initialisation");
        }
        sync
    };
    /// Location of the message queue
    /// Only valid if `entries::init()` was called prior to the first acess
    pub static ref MESSAGE_QUEUE: usize = {
        let queue = MEM_ADDRS.read().message_queue;
        if queue == 0 {
            panic!("Tried to read from MEM_ADDRS prior to initialisation");
        }
        queue
    };
    /// Location of the resource table
    /// Only valid if `entries::init()` was called prior to the first acess
    pub static ref RESOURCE_TABLE: usize = {
        let res = MEM_ADDRS.read().resource_table;
        if res == 0 {
            panic!("Tried to read from MEM_ADDRS prior to initialisation");
        }
        res
    };
    /// Location of the heap
    /// Only valid if `entries::init()` was called prior to the first acess
    pub static ref HEAP_BASE: usize = {
        let heap = MEM_ADDRS.read().resource_table;
        if heap == 0 {
            panic!("Tried to read from MEM_ADDRS prior to initialisation");
        }
        heap
    };
}

#[derive(Debug, Clone)]
#[repr(C)]
/// The MemoryAdresses struct contains the offsets to the staically alocated memory constructs
pub struct MemoryAdresses {
    pub synchronization_memory: usize,
    pub message_queue: usize,
    pub resource_table: usize,
    pub heap_base: usize,
}
impl MemoryAdresses {
    //create dummy memory
    const fn empty() -> Self {
        Self {
            synchronization_memory: 0,
            message_queue: 0,
            resource_table: 0,
            heap_base: 0,
        }
    }
    /// The first section of our heap is used to place shared memory constructs
    /// This function generates the memory positions for those structs and calculates the real
    /// heap_base
    pub fn init(heap_base: u32) {
        let synchronization_memory = align_up::<SynchronizationMemory>(heap_base as usize);
        let message_queue = align_up::<MessageQueueElement<Message>>(
            synchronization_memory + size_of::<SynchronizationMemory>(),
        );
        let resource_table = align_up::<Resource>(message_queue + MESSAGE_QUEUE_SIZE);
        let heap_base = align_up::<u32>(resource_table + RESOURCE_TABLE_SIZE);
        let mem = Self {
            synchronization_memory,
            message_queue,
            resource_table,
            heap_base,
        };
        *(MEM_ADDRS.write()) = mem;
    }
}

#[repr(align(4))]
#[repr(C)]
/// The SynchronizationMemory is the main interface to the main.js
/// It exposes some variables from the main.js and is used for the atomic wait cycle
pub struct SynchronizationMemory {
    /// Time elapsed since logic thread initialisation in milliseconds
    pub elapsed_ms: i32,
    /// Current mouse position
    pub mouse: (i32, i32),
    /// Canvas size in px
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
        &*(*SYNCHRONIZATION_MEMORY as *const Self)
    }
    /// # Safety
    /// This function is safe, if the SYNCHRONIZATION_MEMORY memory address is valid
    /// and is only written to using atomic operations
    pub unsafe fn get_mut() -> &'static mut Self {
        &mut *(*SYNCHRONIZATION_MEMORY as *mut Self)
    }

    /// This functions lets the thread sleep until it is woken up by the main.js or a timeout is
    /// reached
    pub fn wait_for_main_thread_notify(&mut self) {
        self.last_elapsed_ms = self.elapsed_ms;
        while self.last_elapsed_ms
            == unsafe { atomic_read_i32(*SYNCHRONIZATION_MEMORY as *const i32) }
        {
            unsafe { wait_until_wake_up_at(*SYNCHRONIZATION_MEMORY as *mut i32) }
        }
    }
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

#[allow(unused_variables)]
/// # Safety
/// This function is safe as long the thread waits at a valid memory address
pub unsafe fn wait_until_wake_up_at(ptr: *mut i32) {
    let timeout = 5;
    #[cfg(target_arch = "wasm32")]
    {
        let res = core::arch::wasm32::i32_atomic_wait(
            ptr,
            atomic_read_i32(ptr),
            1000 * 1000 * 1000 * timeout,
        );
        if res != 0 {
            log::trace!("Thread woke up after {}e", timeout);
        }
    }
}
