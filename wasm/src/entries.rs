//! This module contains the entry points callable from JavaScript.
//!
//! # Usage
//!
//! To initialize the memory correctly, the exports have to be called in the following order:
//! 1. `exports.__wasm_init_memory()`
//! 2. `exports.__wasm_init_tls()`
//! 3. `exports.init()`
//! Only now other functions may get called.
//! Calling 1-3 more than once is undefined behavior.
//! When executing `init()` a message is sent to the main thread, signaling the initialization has
//! finished. This signal is used to start the graphics worker.

use crate::graphics::renderer;
use crate::logic::LogicContext;
#[cfg(target_arch = "wasm32")]
use crate::{
    communication::{
        Message, MessageQueue, SynchronizationMemory, MESSAGE_QUEUE_ELEMENT_COUNT,
        SYNCHRONIZATION_MEMORY,
    },
    mem,
    wasm_log::{init_panic_handler, WasmLog},
};
use linked_list_allocator::LockedHeap;

use nobg_web_worker::child_entry_point;
#[cfg(target_arch = "wasm32")]
use nobg_web_worker::set_global_thread_pool;

#[cfg(target_arch = "wasm32")]
static LOGGER: WasmLog = WasmLog;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn spawn_graphics_worker(stack_top: u32, tls: u32);
}

/// This function initializes the heap, logger, panic handler and graphics context.
/// The sizes of static elements such as the resource_table can be set in `crate::mem`.
///
/// # Safety
///
/// This function may only be called once at the start of the program.
/// Any call to alloc prior to this functions invocation results in an error.
#[export_name = "init"]
#[cfg(target_arch = "wasm32")]
pub extern "C" fn init(heap_base: u32, mem_size: u32, tls_size: u32) -> u32 {
    unsafe {
        mem::set_tls_size(tls_size);
        ALLOCATOR
            .lock()
            .init(heap_base as usize, (mem_size - heap_base) as usize);
    }
    // set custom panic handler
    init_panic_handler();
    log::set_logger(&LOGGER).unwrap();
    // change the log level to only show certain errors
    log::set_max_level(log::LevelFilter::Debug);
    log::info!("mem_size: {}", mem_size - heap_base);
    mem::alloc_tls() as u32
}

/// Initialize the game state, communicate with the graphics worker and set up networking.
/// This function is being exposed to JavaScript.
#[export_name = "run_logic"]
pub extern "C" fn run_logic() {
    // create the logic game context
    let mut game = LogicContext::new().unwrap_or_else(|e| panic!("{}", e));

    #[cfg(target_arch = "wasm32")]
    {
        let syn_addr = unsafe { &SYNCHRONIZATION_MEMORY as *const SynchronizationMemory as u32 };
        // send memory offset to the main thread -> initialize graphics
        Message::Memory(
            syn_addr,
            &game.message_queue as *const MessageQueue as u32,
            MESSAGE_QUEUE_ELEMENT_COUNT as u32,
        )
        .send();
        let stack = mem::alloc_stack();
        let tls = mem::alloc_tls();
        log::debug!("spawn graphic");
        unsafe {
            spawn_graphics_worker(stack as u32, tls as u32);
        }
        let _global_worker_pool =
            set_global_thread_pool(4, 1024 * 64 * 8, mem::get_tls_size() as u32).unwrap();
    }
    loop {
        game.tick()
            .unwrap_or_else(|e| log::error!("Error occurred game_context.tick(): {:?}", e));
        log::trace!("wait_for_main_thread_notify()");
        // use wasm's atomic wait instruction to sleep until waken by the main thread
        #[cfg(target_arch = "wasm32")]
        unsafe {
            SYNCHRONIZATION_MEMORY.wait_for_main_thread_notify()
        };
    }
}

/// This function is called to render each frame.
/// Most of the communication with the graphics API is done through calling JS functions.
#[export_name = "draw_frame"]
pub extern "C" fn draw_frame() {
    match unsafe { renderer::renderer_mut() } {
        // create a new graphics context if there is none, this persists local data across `draw_frame` invocations
        None => unsafe {
            renderer::set_renderer(
                renderer::Renderer::new()
                    .map_err(|e| panic!("{}", e))
                    .unwrap(),
            )
        },
        Some(ctx) => ctx,
    }
    .render()
    .unwrap_or_else(|e| log::error!("{}", e));
}

/// This function serves as the entry point for threadpool workers
#[export_name = "run_pool"]
pub unsafe extern "C" fn run_pool(ptr: u32) {
    child_entry_point(ptr);
}
